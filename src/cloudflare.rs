use crate::dns::RecordType;
use reqwest::{header::HeaderMap, Client, ClientBuilder, RequestBuilder};
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::net::IpAddr;

pub struct Cloudflare {
    client: Client,
    zone_id: String,
}

impl Cloudflare {
    pub fn new<S: Into<String>>(email: S, key: S, zone_id: S) -> reqwest::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Auth-Email", email.into().parse().unwrap());
        headers.insert("X-Auth-Key", key.into().parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        Ok(Self {
            client: ClientBuilder::new().default_headers(headers).build()?,
            zone_id: zone_id.into(),
        })
    }

    /// Get a list of records, filted by `name` and `type`
    pub fn get_record<S: AsRef<str>>(&self, name: S, record_type: &RecordType) -> RequestBuilder {
        self.client.get(
            format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type={},name={},match=all",
                self.zone_id, record_type.to_string(), name.as_ref()
            )
            .as_str(),
        )
    }

    /// Update a record on cloudflare
    pub fn update_record(&self, record: &UpdateRecord) -> RequestBuilder {
        self.client
            .put(
                format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    self.zone_id, record.id
                )
                .as_str(),
            )
            .json(record)
    }

    /// Get current public ip.
    pub fn public_ip(&self) -> RequestBuilder {
        self.client.get("https://canihazip.com/s")
    }
}

/// Cloudflare return record type
#[derive(Deserialize)]
pub struct Record {
    pub id: String,
    pub record_type: RecordType,
    pub name: String,
    pub content: IpAddr,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: usize,
    pub locked: bool,
    pub zone_id: String,
    pub zone_name: String,
    created_on: IgnoredAny,
    modified_on: IgnoredAny,
    pub data: String,
}

/// Cloudflare response
#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<String>,
    pub result: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRecord {
    id: String,
    record_type: RecordType,
    name: String,
    content: IpAddr,
    ttl: u32,
    proxied: Option<bool>,
}

impl UpdateRecord {
    pub fn new<I: Into<String>, N: Into<String>>(
        id: I,
        record_type: RecordType,
        name: N,
        content: IpAddr,
        ttl: u32,
        proxied: Option<bool>,
    ) -> Self {
        Self {
            id: id.into(),
            record_type,
            name: name.into(),
            content,
            ttl,
            proxied,
        }
    }
}
