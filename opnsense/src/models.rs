use std::collections::HashMap;

use serde::de;
use serde::Serializer;
use serde::{Deserialize, Deserializer, Serialize};

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
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
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
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
    pub enabled: bool,
    pub hostname: String,
    pub domain: String,
    pub rr: HostOverrideType,
    pub mxprio: String,
    pub mx: String,
    pub server: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HostAliasRow {
    pub uuid: String,
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
    pub enabled: bool,
    pub host: String,
    pub hostname: String,
    pub domain: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
    pub enabled: bool,
    pub hostname: String,
    pub host: String,
}

pub type Uuid = String;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResult {
    pub result: String,
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing)]
    pub validations: Option<HashMap<String, String>>,
}

fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let res = match *value {
        true => "1",
        false => "0",
    };

    serializer.serialize_str(res)
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

// // A custom deserialization function, referred to by `deserialize_with`.
// // (reference: https://github.com/serde-rs/serde/issues/1158)
// fn all<'de, D>(deserializer: D) -> Result<(), D::Error>
// where
//     D: Deserializer<'de>,
// {
//     #[derive(Deserialize)]
//     // This enum is, by default, "externally tagged";
//     // but, since it consists only of a single unit variant,
//     // it means that it can be deserialized only from
//     // the corresponding constant string - and that's exactly what we need
//     enum Helper {
//         #[serde(rename = "all")]
//         Variant,
//     }
//     // We're not interested in the deserialized value (we know what it is),
//     // so we can simply map it to (), as required by signature
//     Helper::deserialize(deserializer).map(|_| ())
// }
