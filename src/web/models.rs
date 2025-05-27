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
pub struct ProviderSpecific {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    #[serde(rename = "dnsName")]
    pub dns_name: String,
    pub targets: Vec<String>,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    #[serde(rename = "recordTTL")]
    pub record_ttl: i64,
    pub labels: HashMap<String, String>,
    #[serde(rename = "providerSpecific")]
    pub provider_specifc: Vec<ProviderSpecific>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    #[serde(rename = "dnsName")]
    pub dns_name: String,
    pub targets: Vec<String>,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    #[serde(rename = "recordTTL")]
    pub record_ttl: i64,
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "providerSpecific")]
    pub provider_specific: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRecords {
    #[serde(rename = "Create")]
    pub create: Vec<Record>,
    #[serde(rename = "UpdateOld")]
    pub update_old: Vec<Record>,
    #[serde(rename = "UpdateNew")]
    pub update_new: Vec<Record>,
    #[serde(rename = "Delete")]
    pub delete: Vec<Record>,
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
