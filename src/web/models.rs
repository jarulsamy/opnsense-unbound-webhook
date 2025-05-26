use opnsense::models::{HostAliasRow, HostOverrideRow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filters {
    pub filters: Vec<String>,
}

impl Filters {
    pub fn new(domain: &Vec<String>) -> Self {
        Self {
            filters: domain.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RecordType {
    CNAME,
    A,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProviderSpecific {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    #[serde(rename = "dnsName")]
    dns_name: String,
    targets: Vec<String>,
    #[serde(rename = "recordType")]
    record_type: RecordType,
    #[serde(rename = "recordTTL")]
    record_ttl: i64,
    labels: HashMap<String, String>,
    #[serde(rename = "providerSpecific")]
    provider_specifc: Vec<ProviderSpecific>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    #[serde(rename = "dnsName")]
    dns_name: String,
    targets: Vec<String>,
    #[serde(rename = "recordType")]
    record_type: RecordType,
    #[serde(rename = "recordTTL")]
    record_ttl: i64,
    labels: Option<HashMap<String, String>>,
    #[serde(rename = "providerSpecific")]
    provider_specific: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRecords {
    #[serde(rename = "Create")]
    create: Vec<Record>,
    #[serde(rename = "UpdateOld")]
    update_old: Vec<Record>,
    #[serde(rename = "UpdateNew")]
    update_new: Vec<Record>,
    #[serde(rename = "Delete")]
    delete: Vec<Record>,
}

impl From<&HostOverrideRow> for Record {
    fn from(value: &HostOverrideRow) -> Self {
        Record {
            dns_name: format!("{}.{}", value.hostname, value.domain),
            targets: vec![value.server.clone()],
            record_type: RecordType::A,
            record_ttl: 60,
            labels: None,
            provider_specific: None,
        }
    }
}

impl From<&HostAliasRow> for Record {
    fn from(value: &HostAliasRow) -> Self {
        Record {
            dns_name: format!("{}.{}", value.hostname, value.domain),
            targets: vec![value.host.clone()],
            record_type: RecordType::CNAME,
            record_ttl: 60,
            labels: None,
            provider_specific: None,
        }
    }
}
