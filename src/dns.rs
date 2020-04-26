use std::fmt;

pub enum DNSType {
    A,
    AAAA,
    CAA,
    CERT,
    CNAME,
    DNSKEY,
    DS,
    LOC,
    NAPTR,
    NS,
    PTR,
    SMIMEA,
    SPF,
    SRV,
    SSHPF,
    TLSA,
    TXT,
    URI,
}

impl From<&str> for DNSType {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::A,
            "AAAA" => Self::AAAA,
            "CAA" => Self::CAA,
            "CERT" => Self::CERT,
            "CNAME" => Self::CNAME,
            "DNSKEY" => Self::DNSKEY,
            "DS" => Self::DS,
            "LOC" => Self::LOC,
            "NAPTR" => Self::NAPTR,
            "NS" => Self::NS,
            "PTR" => Self::PTR,
            "SMIMEA" => Self::SMIMEA,
            "SPF" => Self::SPF,
            "SRV" => Self::SRV,
            "SSHPF" => Self::SSHPF,
            "TLSA" => Self::TLSA,
            "TXT" => Self::TXT,
            "URI" => Self::URI,
            _ => panic!("Invalid DNS record type"),
        }
    }
}

impl fmt::Display for DNSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::AAAA => write!(f, "AAAA"),
            Self::CAA => write!(f, "CAA"),
            Self::CERT => write!(f, "CERT"),
            Self::CNAME => write!(f, "CNAME"),
            Self::DNSKEY => write!(f, "DNSKEY"),
            Self::DS => write!(f, "DS"),
            Self::LOC => write!(f, "LOC"),
            Self::NAPTR => write!(f, "NAPTR"),
            Self::NS => write!(f, "NS"),
            Self::PTR => write!(f, "PTR"),
            Self::SMIMEA => write!(f, "SMIMEA"),
            Self::SPF => write!(f, "SPF"),
            Self::SRV => write!(f, "SRV"),
            Self::SSHPF => write!(f, "SSHPF"),
            Self::TLSA => write!(f, "TLSA"),
            Self::TXT => write!(f, "TXT"),
            Self::URI => write!(f, "URI"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_type() {
        for record_type in [
            "A", "AAAA", "CAA", "CERT", "CNAME", "DNSKEY", "DS", "LOC", "NAPTR", "NS", "PTR",
            "SMIMEA", "SPF", "SRV", "SSHPF", "TLSA", "TXT", "URI",
        ]
        .iter()
        {
            assert_eq!(DNSType::from(*record_type).to_string(), record_type.to_string())
        }
    }

    #[test]
    #[should_panic]
    fn test_bad_dns_type() {
        DNSType::from("PANIC");
    }
}
