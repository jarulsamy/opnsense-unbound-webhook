use serde::de;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum HostOverrideType {
    #[serde(rename = "A (IPv4 address)")]
    A,
    #[serde(rename = "AAAA (IPv6 address)")]
    AAAA,
    #[serde(rename = "MX (Mail server)")]
    MX,
}

impl Serialize for HostOverrideType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            HostOverrideType::A => "A",
            HostOverrideType::AAAA => "AAAA",
            HostOverrideType::MX => "MX",
        };
        serializer.serialize_str(s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostOverrideRow {
    uuid: String,
    #[serde(deserialize_with = "deserialize_bool")]
    enabled: bool,
    hostname: String,
    domain: String,
    rr: HostOverrideType,
    server: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostOverride {
    rows: Vec<HostOverrideRow>,
    #[serde(rename = "rowCount")]
    row_count: u64,
    total: u64,
    current: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewHostOverride {
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub hostname: String,
    pub domain: String,
    pub rr: HostOverrideType,
    pub mxprio: String,
    pub mx: String,
    pub server: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostAliasRow {
    uuid: String,
    #[serde(deserialize_with = "deserialize_bool")]
    enabled: bool,
    host: String,
    hostname: String,
    domain: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostAlias {
    rows: Vec<HostAliasRow>,
    #[serde(rename = "rowCount")]
    row_count: u64,
    total: u64,
    current: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewHostAlias {
    pub description: String,
    pub domain: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub hostname: String,
    pub host: String,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["1", "0"])),
    }
}
