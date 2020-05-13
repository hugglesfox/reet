extern crate clokwerk;
extern crate reqwest;
extern crate serde;
extern crate tokio;

mod cloudflare;
mod config;
mod dns;
mod reet;

use cloudflare::Cloudflare;
use config::Config;
use reet::Reet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = Config::new("REET");
    let cloudflare = Cloudflare::new(
        config.cloudflare_email().expect("Missing Cloudflare email"),
        config
            .cloudflare_api_key()
            .expect("Missing Cloudflare API key"),
        config.zone_id().expect("Missing Cloudflare zone ID"),
    )
    .expect("Error creating HTTP client");

    let frequency = Duration::from_secs(config.frequency().unwrap_or(300).into());
    let app = Reet::new(config, cloudflare);

    let records = app.parse_records().await;

    loop {
        app.update(&records).await;
        std::thread::sleep(frequency)
    }
}
