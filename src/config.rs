use crate::dns::RecordType;
use std::env;
use std::net::IpAddr;

struct Config<'a> {
    prefix: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self {
            prefix: prefix,
        }
    }

    fn vars(&self) -> impl Iterator<Item = (String, String)> + '_ {
        env::vars().filter(move |(n, _)| n.starts_with(self.prefix))
    }

    fn get_vars_by<S: 'a + AsRef<str>>(
        &self,
        suffix: S,
    ) -> impl Iterator<Item = (String, String)> + '_ {
        self.vars()
            .filter(move |(n, _)| n.ends_with(suffix.as_ref()))
    }

    fn get_var_by<S: 'a + AsRef<str>>(&self, suffix: S) -> Option<String> {
        self.get_vars_by(suffix).next().map(|t| t.1)
    }

    /// Get all the `REET_*_NAME` environment variables
    pub fn names(&self) -> Vec<(String, String)> {
        self.get_vars_by("_NAME").collect()
    }

    /// Get all the `REET_*_TYPE` environment variables
    pub fn types(&self) -> Vec<(String, RecordType)> {
        self.get_vars_by("_TYPE")
            .map(move |(n, v)| (n, v.parse().expect("Invalid DNS record type")))
            .collect()
    }

    /// Get all the `REET_*_IP` environment variables
    pub fn ip(&self) -> Vec<(String, IpAddr)> {
        self.get_vars_by("_IP")
            .map(move |(n, v)| (n, v.parse().expect("Invalid IP address")))
            .collect()
    }

    /// Get all the `REET_*_TTL` environment variables
    pub fn ttl(&self) -> Vec<(String, i32)> {
        self.get_vars_by("_TTL")
            .map(|(n, v)| (n, v.parse().expect("Invalid integer")))
            .collect()
    }

    /// Get all the `REET_*_PROXIED` environment variables
    pub fn proxied(&self) -> Vec<(String, bool)> {
        self.get_vars_by("_PROXIED")
            .map(|(n, v)| (n, v.parse().expect("Invalid bool")))
            .collect()
    }

    /// Get the `REET_ZONE_ID` environment variable
    pub fn zone_id(&self) -> Option<String> {
        self.get_var_by("_ZONE_ID")
    }

    /// Get the `REET_CLOUDFLARE_EMAIL` encvrionment variable
    pub fn cloudflare_email(&self) -> Option<String> {
        self.get_var_by("_CLOUDFLARE_EMAIL")
    }

    /// Get the `REET_CLOUDFLARE_API_KEY` encvrionment variable
    pub fn cloudflare_api_key(&self) -> Option<String> {
        self.get_var_by("_CLOUDFLARE_API_KEY")
    }

    /// Get the `REET_FREQUENCY` encvrionment variable
    pub fn frequency(&self) -> Option<String> {
        self.get_var_by("_FREQUENCY")
    }
}
