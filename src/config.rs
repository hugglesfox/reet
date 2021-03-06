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

    /// Get the `REET_CLOUDFLARE_API_KEY` encvrionment variable
    pub fn cloudflare_api_key(&self) -> Option<String> {
        self.get_var_by("_CLOUDFLARE_API_KEY")
    }

    /// Get the `REET_FREQUENCY` encvrionment variable
    pub fn frequency(&self) -> Option<u64> {
        self.get_var_by("_FREQUENCY")
            .and_then(|v| v.parse::<u64>().ok())
    }

    /// Get all the `REET_*_NAME` environment variables
    pub fn names(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.get_vars_by("_NAME")
    }

    /// Get all the `REET_*_IP` environment variables
    pub fn ip(&self) -> impl Iterator<Item = (String, Option<IpAddr>)> + '_ {
        self.get_vars_by("_IP")
            .map(move |(n, v)| (n, v.parse::<IpAddr>().ok()))
    }

    /// Get all the `REET_*_TTL` environment variables
    pub fn ttl(&self) -> impl Iterator<Item = (String, Option<u32>)> + '_ {
        self.get_vars_by("_TTL")
            .map(|(n, v)| (n, v.parse::<u32>().ok()))
    }

    /// Get all the `REET_*_PROXIED` environment variables
    pub fn proxied(&self) -> impl Iterator<Item = (String, Option<bool>)> + '_ {
        self.get_vars_by("_PROXIED")
            .map(|(n, v)| (n, v.parse::<bool>().ok()))
    }

    /// Get the `REET_*_IP` variable from a `REET_*_NAME`
    pub fn get_ip<S: 'a + AsRef<str>>(&self, name: S) -> Option<IpAddr> {
        self.ip()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .and_then(|(_, v)| v)
    }

    /// Get the `REET_*_TTL` variable from a `REET_*_NAME`
    pub fn get_ttl<S: 'a + AsRef<str>>(&self, name: S) -> Option<u32> {
        self.ttl()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .and_then(|(_, v)| v)
    }

    /// Get the `REET_*_PROXIED` variable from a `REET_*_NAME`
    pub fn get_proxied<S: 'a + AsRef<str>>(&self, name: S) -> Option<bool> {
        self.proxied()
            .filter(|(n, _)| n.contains(name.as_ref().trim_end_matches("_NAME")))
            .next()
            .and_then(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Config<'static> {
        env::set_var("REETTEST_ZONE_ID", "cloudflare_zone_id");
        env::set_var("REETTEST_CLOUDFLARE_API_KEY", "cloudflare_api_key");
        env::set_var("REETTEST_FREQUENCY", "10");

        env::set_var("REETTEST_FOO_NAME", "example.com");
        env::set_var("REETTEST_FOO_TYPE", "A");
        env::set_var("REETTEST_FOO_IP", "127.0.0.1");
        env::set_var("REETTEST_FOO_TTL", "120");

        env::set_var("REETTEST_BAR_NAME", "example.org");
        env::set_var("REETTEST_BAR_TYPE", "AAAA");
        env::set_var("REETTEST_BAR_PROXIED", "true");

        env::set_var("REETTEST_FIZZ_IP", "::1");

        Config::new("REETTEST")
    }

    #[test]
    fn test_vars() {
        let config = setup();

        assert_eq!(
            config
                .vars()
                .filter(|(name, _)| !name.starts_with("REETTEST"))
                .collect::<Vec<(String, String)>>(),
            vec![]
        )
    }

    #[test]
    fn test_config() {
        let config = setup();

        assert_eq!(config.zone_id().unwrap(), "cloudflare_zone_id");
        assert_eq!(config.cloudflare_api_key().unwrap(), "cloudflare_api_key");
        assert_eq!(config.frequency().unwrap(), 10);

        assert_eq!(
            config.names().collect::<Vec<(String, String)>>(),
            vec![
                ("REETTEST_FOO_NAME", "example.com"),
                ("REETTEST_BAR_NAME", "example.org")
            ]
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect::<Vec<(String, String)>>()
        );
        assert_eq!(
            config.get_ip("REETTEST_FOO_NAME").unwrap(),
            "127.0.0.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(config.get_ttl("REETTEST_FOO_NAME").unwrap(), 120);
        assert_eq!(config.get_proxied("REETTEST_FOO_NAME"), None);

        assert_eq!(config.get_ip("REETTEST_BAR_NAME"), None);
        assert_eq!(config.get_ttl("REETTEST_BAR_NAME"), None);
        assert_eq!(config.get_proxied("REETTEST_BAR_NAME").unwrap(), true);

        assert_eq!(
            config.get_ip("REETTEST_FIZZ_IP").unwrap(),
            "::1".parse::<IpAddr>().unwrap()
        );
    }
}
