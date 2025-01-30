mod api;
pub mod client;
mod models;
mod types;
mod workflow;

use crate::types::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build()?;

    let resy_config: ResyConfig = settings.try_deserialize()?;
    let reservation_details: ReservationDetails = resy_config.reservation_details;

    println!("Configuration loaded successfully!");

    let api = api::ResyApi::new(resy_config.resy_keys);
    let client = client::ResyClient::new(api);
    let workflow =
        workflow::ResyBookingWorkflow::new(client, reservation_details, resy_config.grab_time);

    workflow.run().await
}
