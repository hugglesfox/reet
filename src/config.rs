use crate::dns::RecordType;
use std::env;
use std::net::IpAddr;

pub struct Config<'a> {
    prefix: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self { prefix }
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
    pub fn frequency(&self) -> Option<u32> {
        self.get_var_by("_FREQUENCY")
            .map(|v| v.parse().expect("Invalid integer"))
    }

    /// Get all the `REET_*_NAME` environment variables
    pub fn names(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.get_vars_by("_NAME")
    }

    /// Get all the `REET_*_TYPE` environment variables
    pub fn types(&self) -> impl Iterator<Item = (String, RecordType)> + '_ {
        self.get_vars_by("_TYPE")
            .map(move |(n, v)| (n, v.parse().expect("Invalid DNS record type")))
    }

    /// Get all the `REET_*_IP` environment variables
    pub fn ip(&self) -> impl Iterator<Item = (String, IpAddr)> + '_ {
        self.get_vars_by("_IP")
            .map(move |(n, v)| (n, v.parse().expect("Invalid IP address")))
    }

    /// Get all the `REET_*_TTL` environment variables
    pub fn ttl(&self) -> impl Iterator<Item = (String, u32)> + '_ {
        self.get_vars_by("_TTL")
            .map(|(n, v)| (n, v.parse().expect("Invalid integer")))
    }

    /// Get all the `REET_*_PROXIED` environment variables
    pub fn proxied(&self) -> impl Iterator<Item = (String, bool)> + '_ {
        self.get_vars_by("_PROXIED")
            .map(|(n, v)| (n, v.parse().expect("Invalid bool")))
    }

    /// Get the `REET_*_TYPE` variable from a `REET_*_NAME`
    pub fn get_type<S: 'a + AsRef<str>>(&self, name: S) -> Option<RecordType> {
        self.types()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .map(|(_, v)| v)
    }

    /// Get the `REET_*_IP` variable from a `REET_*_NAME`
    pub fn get_ip<S: 'a + AsRef<str>>(&self, name: S) -> Option<IpAddr> {
        self.ip()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .map(|(_, v)| v)
    }

    /// Get the `REET_*_TTL` variable from a `REET_*_NAME`
    pub fn get_ttl<S: 'a + AsRef<str>>(&self, name: S) -> Option<u32> {
        self.ttl()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .map(|(_, v)| v)
    }

    /// Get the `REET_*_PROXIED` variable from a `REET_*_NAME`
    pub fn get_proxied<S: 'a + AsRef<str>>(&self, name: S) -> Option<bool> {
        self.proxied()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .map(|(_, v)| v)
    }
}

}
