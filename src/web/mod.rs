mod models;

use crate::web::models::RecordType;
use opnsense::models::HostOverrideType;
use opnsense::models::NewHostOverride;
use rocket::State;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;

const RECORD_DESCRIPTION_PREFIX: &'static str = "_ouw_";

#[derive(Responder)]
#[response(
    status = 200,
    content_type = "application/external.dns.webhook+json;version=1"
)]
pub struct WebhookJson<T>(pub Json<T>);

#[get("/healthz")]
pub fn healthz() -> &'static str {
    "OK"
}

#[get("/")]
pub fn negotiate(domains: &State<Vec<String>>) -> WebhookJson<models::Filters> {
    let f = models::Filters::new(&domains);
    WebhookJson(Json(f))
}

#[get("/records")]
pub async fn records_get(opnsense: &State<opnsense::Opnsense>) -> WebhookJson<Vec<models::Record>> {
    //  Host Overrides <-> A records
    //  Host Aliases   <-> CName records
    let host_overrides = opnsense.unbound_get_host_overrides().await.unwrap();
    let host_aliases = opnsense.unbound_get_host_aliases().await.unwrap();

    let mut resp: Vec<models::Record> = vec![];
    for row in &host_overrides.rows {
        if !row.enabled || !row.description.starts_with(RECORD_DESCRIPTION_PREFIX) {
            continue;
        }
        let record: models::Record = row.into();
        resp.push(record);
    }

    for row in &host_aliases.rows {
        if !row.enabled || !row.description.starts_with(RECORD_DESCRIPTION_PREFIX) {
            continue;
        }
        let record: models::Record = row.into();
        resp.push(record);
    }

    WebhookJson(Json(resp))
}

fn dns_name_to_hostname_and_domain(dns_name: &str) -> Option<(String, String)> {
    match dns_name.split_once(".") {
        Some((first, rest)) => Some((first.to_string(), rest.to_string())),
        None => None,
    }
}

#[post("/records", format = "json", data = "<body>")]
pub async fn records_post(
    opnsense: &State<opnsense::Opnsense>,
    body: Json<models::UpdateRecords>,
) -> Status {
    let records = body.into_inner();
    // match validate_records(&records) {
    //     Err(_) => return Status::InternalServerError,
    //     Ok(_) => {}
    // }

    // let overrides = opnsense.unbound_get_host_overrides().await.unwrap();
    // let aliases = opnsense.unbound_get_host_aliases().await.unwrap();

    for i in &records.create {
        match i.record_type {
            RecordType::A => {
                let (hostname, domain) = match dns_name_to_hostname_and_domain(&i.dns_name) {
                    Some((hostname, domain)) => (hostname, domain),
                    None => return Status::InternalServerError,
                };
                for server in &i.targets {
                    let payload = NewHostOverride {
                        enabled: true,
                        hostname: hostname.clone(),
                        domain: domain.clone(),
                        rr: HostOverrideType::A,
                        mxprio: "".to_string(),
                        mx: "".to_string(),
                        server: server.to_string(),
                        description: "_ouw_".to_string(),
                    };
                    opnsense.unbound_add_host_override(&payload).await.unwrap();
                }
            }
            RecordType::CNAME => {}
        }
    }
    for i in &records.update_old {
        info!("Update Old: {:?}", i)
    }
    for i in &records.update_new {
        info!("Update New: {:?}", i)
    }
    for i in &records.delete {
        info!("Delete {:?}", i)
    }

    Status::NoContent
    // Sample Request
    // curl -X POST http://localhost:8000/records \
    //   -H "Content-Type: application/json" \
    //   -d '{
    //     "Create": [
    //       {
    //         "dnsName": "example.yourdomain.com.",
    //         "targets": ["203.0.113.42"],
    //         "recordType": "A",
    //         "recordTTL": 300,
    //         "labels": {
    //           "external-dns/owner": "default",
    //           "external-dns/resource": "service/default/my-service"
    //         }
    //       }
    //     ],
    //     "UpdateOld": [],
    //     "UpdateNew": [],
    //     "Delete": []
    //   }'
}

#[post("/adjustendpoints", format = "json", data = "<body>")]
pub fn adjust_endpoints(body: Json<Vec<models::Record>>) -> WebhookJson<Vec<models::Record>> {
    // Assume all transactions are valid.
    // TODO: Properly validate records.
    WebhookJson(body)
}
