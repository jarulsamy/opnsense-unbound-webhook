#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

mod models;

#[get("/")]
fn negotiate() -> Json<models::Filters> {
    let f = models::Filters::new(".arulsamy.me");
    Json(f)
}

#[get("/records")]
fn records_get() -> &'static str {
    todo!();
}

#[post("/records")]
fn records_post() {
    todo!();
}

#[post("/adjustendpoints")]
fn adjust_endpoints() -> () {
    todo!();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![negotiate, records_get, records_post, adjust_endpoints],
        )
        .launch()
        .await?;

    Ok(())
}
