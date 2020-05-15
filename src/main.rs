extern crate cloudflare;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate canihazip;
extern crate dotenv;
extern crate human_panic;

mod config;

use crate::cloudflare::framework::apiclient::ApiClient;
use cloudflare::endpoints::dns::{
    DnsContent, ListDnsRecords, ListDnsRecordsParams, UpdateDnsRecord, UpdateDnsRecordParams,
};
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::{Environment, HttpApiClient, HttpApiClientConfig, SearchMatch};
use config::Config;
use human_panic::setup_panic;
use std::net::IpAddr;
use std::time::Duration;

const PREFIX: &'static str = "REET";

fn main() {
    // Import envvars from .env
    dotenv::dotenv().ok();

    // Setup logging
    setup_panic!();
    pretty_env_logger::init_custom_env(format!("{}_LOG_LEVEL", PREFIX).as_str());

    let config = Config::new(PREFIX);
    let frequency = Duration::from_secs(config.frequency().unwrap_or(300));

    let credentials = Credentials::UserAuthToken {
        token: match config.cloudflare_api_key() {
            Some(v) => v,
            None => {
                error!("Unable to get cloudflare api key");
                std::process::exit(1);
            }
        },
    };

    let zone_identifier = match config.zone_id() {
        Some(id) => id,
        None => {
            error!("Unable to get zone id");
            std::process::exit(1);
        }
    };

    let client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        Environment::Production,
    )
    .expect("Unable to create cloudflare client");

    loop {
        for (name, value) in config.names() {
            // Get IP
            let ip = match config.get_ip(&name) {
                Some(c) => c,
                None => {
                    warn!("Unable to get ip for {} from environment variables, reverting to public IP", value);
                    match canihazip::plz_ipv4() {
                        Ok(ip) => IpAddr::from(ip),
                        Err(e) => {
                            error!("Failed to aquire a public Ipv4 address: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            };

            // Create config vars
            let content = match ip {
                IpAddr::V4(c) => DnsContent::A { content: c },
                IpAddr::V6(c) => DnsContent::AAAA { content: c },
            };

            info!("Updating {} to {:?}", value, ip);

            // Query record from cloudflare to get the record id.
            // Also means that we can check if the record exists before trying to update it.
            let list_params = ListDnsRecordsParams {
                record_type: None,
                name: Some(value.to_string()),
                page: None,
                per_page: None,
                order: None,
                direction: None,
                search_match: Some(SearchMatch::All),
            };

            let list_record = ListDnsRecords {
                zone_identifier: &zone_identifier,
                params: list_params,
            };

            let records = match client.request(&list_record) {
                Ok(response) => response.result,
                Err(e) => {
                    error!("Unable to get the dns record {} {:?}: {}", value, ip, e);
                    continue;
                }
            };

            // Update record.
            let update_params = UpdateDnsRecordParams {
                name: &value,
                content: content,
                ttl: config.get_ttl(&name),
                proxied: config.get_proxied(&name),
            };

            let update_record = UpdateDnsRecord {
                zone_identifier: &zone_identifier,
                identifier: match records
                    .iter()
                    .filter(|r| match &r.content {
                        DnsContent::A { content: _ } => ip.is_ipv4(),
                        DnsContent::AAAA { content: _ } => ip.is_ipv6(),
                        _ => false,
                    })
                    .next() // HACK: Reqwest wasn't likeing the DnsContent struct so DNS record type filtering needed to be done on the client.
                {
                    Some(record) => {
                        debug!("Updating record with id {}", record.id);
                        &record.id
                    }
                    None => {
                        error!("No record found which matches {} ({})", value, name);
                        continue;
                    }
                },
                params: update_params,
            };

            match client.request(&update_record) {
                Ok(_) => info!("Successfully updated {}!", value),
                Err(e) => error!("Unable to update {}: {}", value, e),
            }
        }

        std::thread::sleep(frequency)
    }
}
