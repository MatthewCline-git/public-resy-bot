use anyhow::Result;
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use crate::api::ResyApi;
use crate::models::{BookingDetails, ResyError};

pub struct ResyClient {
    api: ResyApi,
}

impl ResyClient {
    pub fn new(api: ResyApi) -> Self {
        Self { api }
    }

    pub async fn find_reservations(
        &self,
        date: &str,
        party_size: i32,
        venue_id: i32,
        earliest_reservation_time: String,
        latest_reservation_time: String,
        accept_outdoor: bool,
        millis_to_retry: u64,
    ) -> Result<String> {
        let start_time = std::time::Instant::now();

        while start_time.elapsed().as_millis() as u64 <= millis_to_retry {
            match self
                .try_find_reservation(
                    date,
                    party_size,
                    venue_id,
                    &earliest_reservation_time,
                    &latest_reservation_time,
                    accept_outdoor,
                )
                .await
            {
                Ok(config_id) => return Ok(config_id),
                Err(ResyError::NoAvailableReservations) => {
                    println!("No reservations found, retrying...");
                    let sleep_duration = if start_time.elapsed().as_secs() < 3 {
                        let jitter = rand::thread_rng().gen_range(40..60);
                        Duration::from_millis(jitter)
                    } else {
                        let jitter = rand::thread_rng().gen_range(400..600);
                        Duration::from_millis(jitter)
                    };

                    sleep(sleep_duration).await;
                    continue;
                }
                Err(e) => return Err(anyhow::anyhow!("{:?}", e)),
            }
        }

        Err(anyhow::anyhow!("Timed out trying to find reservation"))
    }

    async fn try_find_reservation(
        &self,
        date: &str,
        party_size: i32,
        venue_id: i32,
        earliest_reservation_time: &String,
        latest_reservation_time: &String,
        accept_outdoor: bool,
    ) -> Result<String, ResyError> {
        let response = self
            .api
            .get_reservations(date, party_size, venue_id)
            .await
            .map_err(|e| ResyError::UnknownError(e.to_string()))?;

        let reservation_map = self.build_reservation_map(&response)?;
        println!("{:?}", reservation_map);
        self.find_best_reservation_time(
            &reservation_map,
            earliest_reservation_time,
            latest_reservation_time,
            accept_outdoor,
        )
    }

    fn build_reservation_map(
        &self,
        response: &Value,
    ) -> Result<HashMap<String, HashMap<String, String>>, ResyError> {
        let mut reservation_map = HashMap::new();

        /*
        And just like that... he's gone
        */

        Ok(reservation_map)
    }

    fn find_best_reservation_time(
        &self,
        reservation_map: &HashMap<String, HashMap<String, String>>,
        earliest_reservation_time: &String,
        latest_reservation_time: &String,
        accept_outdoor: bool,
    ) -> Result<String, ResyError> {
        println!("\nLooking for reservations...");
        println!("Available times in map: {:?}", reservation_map.keys());

        let mut times: Vec<&String> = reservation_map.keys().collect();
        times.sort();

        for time in times {
            if time < earliest_reservation_time || time > latest_reservation_time {
                continue;
            }

            if let Some(table_types) = reservation_map.get(time) {
                for (table_type, config_id) in table_types {
                    let is_outdoor = table_type.to_lowercase().contains("outdoor")
                        || table_type.to_lowercase().contains("patio");
                    if is_outdoor && !accept_outdoor {
                        continue;
                    }

                    return Ok(config_id.clone());
                }
            }
        }

        Err(ResyError::CannotFindReservation)
    }

    pub async fn get_reservation_details(
        &self,
        config_id: &str,
        date: &str,
        party_size: i32,
    ) -> Result<BookingDetails> {
        let response = self
            .api
            .get_reservation_details(config_id, date, party_size)
            .await?;

        let payment_method_id = response["user"]["payment_methods"][0]["id"]
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("Could not find payment method id"))?;

        let booking_token = response["book_token"]["value"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Could not find booking token"))?;

        Ok(BookingDetails {
            payment_method_id: payment_method_id as i32,
            booking_token: booking_token.to_string(),
        })
    }

    pub async fn book_reservation(
        &self,
        payment_method_id: i32,
        book_token: &str,
    ) -> Result<String> {
        let response = self
            .api
            .post_reservation(payment_method_id, book_token)
            .await?;

        let resy_token = response["resy_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Could not find resy token"))?;

        Ok(resy_token.to_string())
    }
}
