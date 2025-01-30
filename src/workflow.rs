use crate::client::ResyClient;
use crate::types::ReservationDetails;
use anyhow::Result;
use chrono::{DateTime, Local};
use tokio::time::{sleep, Duration, Instant};

pub struct ResyBookingWorkflow {
    client: ResyClient,
    reservation_details: ReservationDetails,
    grab_time: DateTime<Local>,
}

impl ResyBookingWorkflow {
    pub fn new(
        client: ResyClient,
        reservation_details: ReservationDetails,
        grab_time: DateTime<Local>,
    ) -> Self {
        Self {
            client,
            reservation_details,
            grab_time,
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("Starting the resy booking bot");
        self.wait_until_grab_time().await?;
        println!("Wait time is over");
        match self.grab_with_retry().await {
            Ok(resy_token) => {
                println!("Got the res. Token is {}", resy_token);
                Ok(())
            }
            Err(e) => {
                println!("Missed it");
                Err(e)
            }
        }
    }

    async fn wait_until_grab_time(&self) -> Result<()> {
        let now = Local::now();
        println!("now is {:?} and grab time is {:?}", now, self.grab_time);
        if now >= self.grab_time {
            return Ok(());
        }

        let wait_duration = self.grab_time - now;

        sleep(wait_duration.to_std()?).await;
        Ok(())
    }

    async fn grab_with_retry(&self) -> Result<String> {
        let start_time = Instant::now();
        let retry_window = Duration::from_secs(10);

        while start_time.elapsed() < retry_window {
            match self.attempt_reservation().await {
                Ok(token) => return Ok(token),
                Err(e) => {
                    println!("Reservation attempt failed: {}. Retrying...", e);
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(anyhow::anyhow!(
            "Failed to grab reservation within retry window"
        ))
    }

    async fn attempt_reservation(&self) -> Result<String> {
        // Try to find an available reservation
        let config_id = self
            .client
            .find_reservations(
                &self.reservation_details.date,
                self.reservation_details.party_size,
                self.reservation_details.venue_id,
                self.reservation_details.earliest_reservation_time.clone(),
                self.reservation_details.latest_reservation_time.clone(),
                self.reservation_details.accept_outdoor,
                1000, // 1 second retry window for finding a specific time
            )
            .await?;

        // Get the reservation details
        let booking_details = self
            .client
            .get_reservation_details(
                &config_id,
                &self.reservation_details.date,
                self.reservation_details.party_size,
            )
            .await?;

        // Try to book the reservation
        self.client
            .book_reservation(
                booking_details.payment_method_id,
                &booking_details.booking_token,
            )
            .await
    }
}
