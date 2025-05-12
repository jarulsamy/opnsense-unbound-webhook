pub mod models;

use std::collections::HashMap;

use anyhow::{Context, Error, Result};
use base64::{Engine, engine::general_purpose};
use reqwest;
use reqwest::header;

pub struct Opnsense {
    pub url: String,

    client: reqwest::Client,
}

impl Opnsense {
    pub fn new(
        url: &str,
        key: &str,
        secret: &str,
        danger_accept_invalid_certs: bool,
    ) -> Result<Self, Error> {
        // Assemble the header for basic auth
        let auth = format!("{}:{}", key, secret);
        let auth_encoded = format!("Basic {}", general_purpose::STANDARD.encode(auth));
        let mut auth_header_val = header::HeaderValue::from_str(&auth_encoded)?;
        auth_header_val.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            auth_header_val,
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(danger_accept_invalid_certs)
            .default_headers(headers)
            .build()
            .with_context(|| "Failed to create client")?;

        Ok(Opnsense {
            url: url.to_string(),
            client,
        })
    }

    fn url(&self, endpoint: &str) -> String {
        let clean = endpoint.trim_start_matches("/");
        format!("{}/{}", self.url, clean)
    }

    pub async fn unbound_get_status(&self) -> Result<models::Status, Error> {
        const _ENDPOINT: &str = "/api/unbound/service/status";
        let endpoint = self.url(_ENDPOINT);
        let resp = self.client.get(endpoint).send().await?;
        let parsed = resp.json::<models::Status>().await?;
        Ok(parsed)
    }

    pub async fn unbound_get_host_overrides(&self) -> Result<models::HostOverride, Error> {
        const _ENDPOINT: &str = "/api/unbound/settings/searchHostOverride/";
        let endpoint = self.url(_ENDPOINT);
        let resp = self.client.get(endpoint).send().await?;
        let parsed = resp.json::<models::HostOverride>().await?;
        Ok(parsed)
    }

    pub async fn unbound_add_host_override(self, new: &models::NewHostOverride) -> Result<reqwest::Response, Error> {
        const _ENDPOINT: &str = "/api/unbound/settings/addHostOverride/";
        let endpoint = self.url(_ENDPOINT);
        let payload: HashMap<&str, &models::NewHostOverride> = [("host", new)].into_iter().collect();
        let resp = self.client.post(endpoint).json(&payload).send().await?;
        Ok(resp)
    }

    pub async fn unbound_get_host_aliases(&self) -> Result<models::HostAlias, Error> {
        const _ENDPOINT: &str = "/api/unbound/settings/searchHostAlias/";
        let endpoint = self.url(_ENDPOINT);
        let resp = self.client.get(endpoint).send().await?;
        let parsed = resp.json::<models::HostAlias>().await?;
        Ok(parsed)
    }

    pub async fn unbound_add_host_alias(self, new: &models::NewHostAlias) -> Result<reqwest::Response, Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let _ = Opnsense::new("blah", "blah", "blah", true);
    }

    #[test]
    fn test_something() {
        // Request a new server from the pool
        let mut server = mockito::Server::new();

        // Use one of these addresses to configure your client
        let host = server.host_with_port();
        let url = server.url();

        // Create a mock
        let mock = server
            .mock("GET", "/hello")
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body("world")
            .create();

        // Any calls to GET /hello beyond this line will respond with 201, the
        // `content-type: text/plain` header and the body "world".

        // You can use `Mock::assert` to verify that your mock was called
        // mock.assert();
    }
}
