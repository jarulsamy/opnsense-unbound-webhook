use serde::de;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum StatusType {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub status: StatusType,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HostOverrideRow {
    pub uuid: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub hostname: String,
    pub domain: String,
    pub rr: HostOverrideType,
    pub server: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HostOverride {
    pub rows: Vec<HostOverrideRow>,
    #[serde(rename = "rowCount")]
    pub row_count: u64,
    pub total: u64,
    pub current: u64,
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
    pub uuid: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub host: String,
    pub hostname: String,
    pub domain: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostAlias {
    pub rows: Vec<HostAliasRow>,
    #[serde(rename = "rowCount")]
    pub row_count: u64,
    pub total: u64,
    pub current: u64,
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
