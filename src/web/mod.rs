mod models;

use rocket::State;
use rocket::serde::json::Json;

const RECORD_DESCRIPTION_PREFIX: &'static str = "_ouw_";

#[get("/healthz")]
pub fn healthz() -> &'static str {
    "OK"
}

#[get("/")]
pub fn negotiate(domains: &State<Vec<String>>) -> Json<models::Filters> {
    let f = models::Filters::new(&domains);
    Json(f)
}

#[get("/records")]
pub async fn records_get(opnsense: &State<opnsense::Opnsense>) -> Json<Vec<models::Record>> {
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

    Json(resp)
}

#[post("/records", format = "json", data = "<body>")]
pub fn records_post(
    opnsense: &State<opnsense::Opnsense>,
    body: Json<models::UpdateRecords>,
) -> Json<Vec<models::Record>> {
    error!("{:?}", body);
    todo!();
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
pub fn adjust_endpoints(body: Json<Vec<models::Record>>) -> Json<Vec<models::Record>> {
    // Assume all transactions are valid.
    // TODO: Properly validate records.
    body
}
