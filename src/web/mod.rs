mod models;

use rocket::State;
use rocket::serde::json::Json;

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
pub fn records_get(opnsense: &State<opnsense::Opnsense>) -> &'static str {
    todo!();
}

#[post("/records")]
pub fn records_post(opnsense: &State<opnsense::Opnsense>) -> &'static str {
    todo!();
}

#[post("/adjustendpoints")]
pub fn adjust_endpoints(opnsense: &State<opnsense::Opnsense>) -> &'static str {
    todo!();
}
