use crate::cloudflare::{Cloudflare, Response, UpdateRecord};
use crate::config::Config;

pub struct Reet<'a> {
    config: Config<'a>,
    cloudflare: Cloudflare,
}

impl<'a> Reet<'a> {
    pub fn new(config: Config<'a>, cloudflare: Cloudflare) -> Self {
        Self { config: config, cloudflare: cloudflare }
    }

    pub async fn parse_records(&self) -> Vec<UpdateRecord> {
        let mut records: Vec<UpdateRecord> = vec![];

        for (name, value) in self.config.names() {
            let record_type = self
                .config
                .get_type(&name)
                .expect("Unable to find record type");

            let record = self
                .cloudflare
                .get_record(&value, &record_type)
                .send()
                .await
                .expect("Error sending request to the cloudflare API")
                .json::<Response>()
                .await
                .expect("Error parsing cloudflare response")
                .result;

            records.push(UpdateRecord::new(
                &record.first().expect("Unable to find record").id,
                record_type,
                value,
                self.config.get_ip(&name).unwrap_or(
                    self.cloudflare
                        .public_ip()
                        .send()
                        .await
                        .expect("Unable to get public IP")
                        .text()
                        .await
                        .expect("Unable to parse request")
                        .parse()
                        .expect("Unable to parse public IP"),
                ),
                self.config.get_ttl(&name).unwrap_or(1),
                self.config.get_proxied(&name),
            ))
        }

        records
    }


    pub async fn update(&self, records: &Vec<UpdateRecord>) {
        for record in records {
            self.cloudflare.update_record(&record).send().await.expect("Error updating record");
        }
    }
}
