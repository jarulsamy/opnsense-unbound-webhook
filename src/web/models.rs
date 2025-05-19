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
