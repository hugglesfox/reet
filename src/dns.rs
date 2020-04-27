use std::str::FromStr;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum RecordType {
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

impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::AAAA),
            "CAA" => Ok(Self::CAA),
            "CERT" => Ok(Self::CERT),
            "CNAME" => Ok(Self::CNAME),
            "DNSKEY" => Ok(Self::DNSKEY),
            "DS" => Ok(Self::DS),
            "LOC" => Ok(Self::LOC),
            "NAPTR" => Ok(Self::NAPTR),
            "NS" => Ok(Self::NS),
            "PTR" => Ok(Self::PTR),
            "SMIMEA" => Ok(Self::SMIMEA),
            "SPF" => Ok(Self::SPF),
            "SRV" => Ok(Self::SRV),
            "SSHPF" => Ok(Self::SSHPF),
            "TLSA" => Ok(Self::TLSA),
            "TXT" => Ok(Self::TXT),
            "URI" => Ok(Self::URI),
            e => Err(format!("Invalid DNS record type {}", e)),
        }
    }
}

impl fmt::Display for RecordType {
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
    fn test_record_type() {
        for record_type in [
            "A", "AAAA", "CAA", "CERT", "CNAME", "DNSKEY", "DS", "LOC", "NAPTR", "NS", "PTR",
            "SMIMEA", "SPF", "SRV", "SSHPF", "TLSA", "TXT", "URI",
        ]
        .iter()
        {
            assert_eq!(
                record_type.parse::<RecordType>().unwrap().to_string(),
                record_type.to_string()
            )
        }
    }

    #[test]
    #[should_panic]
    fn test_bad_record_type() {
        "PANIC".parse::<RecordType>().unwrap();
    }
}
