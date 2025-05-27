#[macro_use]
extern crate rocket;

extern crate opnsense;

use clap::Parser;
use log::debug;
use std::env;

mod web;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Domains supported by this instance.
    #[arg(short, long = "domain", env)]
    domains: Vec<String>,

    /// Increase log level for more debug info.
    #[arg(long, env, default_value = "info")]
    log_level: log::Level,

    /// Secret to use to authenticate with OPNSense.
    #[arg(long, env)]
    opnsense_secret: Option<String>,

    /// Key to use to authenticate with OPNSense.
    #[arg(long, env)]
    opnsense_key: Option<String>,

    /// URL of OPNSense instance. Must include protocol (http(s)).
    #[arg(long, env)]
    opnsense_url: String,

    /// Ignore HTTPS certificate errors.
    #[arg(long, action, env)]
    insecure: bool,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.log_level.to_level_filter())
        .init();
    debug!("{:?}", args);

    let in_k8s = env::var("KUBERNETES_SERVICE_HOST").is_ok();
    if in_k8s {
        debug!("Determined we're in k8s!")
    }

    for i in &args.domains {
        debug!("Found included domain: {}", &i);
    }

    let opnsense = opnsense::Opnsense::new(
        &args.opnsense_url,
        args.opnsense_key,
        args.opnsense_secret,
        args.insecure,
    )
    .unwrap();

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                web::healthz,
                web::negotiate,
                web::records_get,
                web::records_post,
                web::adjust_endpoints,
            ],
        )
        .manage(args.domains)
        .manage(opnsense)
        .launch()
        .await?;

    Ok(())
}
