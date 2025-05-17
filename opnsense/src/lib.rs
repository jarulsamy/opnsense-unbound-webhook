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

enum ApiEndpoint {
    UnboundServiceStatus,
    UnboundSearchHostOverrides,
    UnboundAddHostOverride,
    UnboundSearchHostAliases,
    UnboundAddHostAlias,
}

impl From<ApiEndpoint> for &'static str {
    fn from(endpoint: ApiEndpoint) -> Self {
        match endpoint {
            ApiEndpoint::UnboundServiceStatus => "/api/unbound/service/status",
            ApiEndpoint::UnboundSearchHostOverrides => "/api/unbound/settings/searchHostOverride/",
            ApiEndpoint::UnboundAddHostOverride => "/api/unbound/settings/addHostOverride/",
            ApiEndpoint::UnboundSearchHostAliases => "/api/unbound/settings/searchHostAlias/",
            ApiEndpoint::UnboundAddHostAlias => "/api/unbound/settings/addHostAlias/",
        }
    }
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
        headers.insert(header::AUTHORIZATION, auth_header_val);
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
        let endpoint: &str = ApiEndpoint::UnboundServiceStatus.into();
        let url = self.url(endpoint);
        let resp = self.client.get(url).send().await?;
        let parsed = resp.json::<models::Status>().await?;
        Ok(parsed)
    }

    pub async fn unbound_get_host_overrides(&self) -> Result<models::HostOverride, Error> {
        let endpoint: &str = ApiEndpoint::UnboundSearchHostOverrides.into();
        let url = self.url(endpoint);
        let resp = self.client.get(url).send().await?;
        let parsed = resp.json::<models::HostOverride>().await?;
        Ok(parsed)
    }

    pub async fn unbound_add_host_override(
        self,
        new: &models::NewHostOverride,
    ) -> Result<reqwest::Response, Error> {
        const _ENDPOINT: &str = "/api/unbound/settings/addHostOverride/";
        let endpoint = self.url(_ENDPOINT);
        let payload: HashMap<&str, &models::NewHostOverride> =
            [("host", new)].into_iter().collect();
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

    pub async fn unbound_add_host_alias(
        self,
        new: &models::NewHostAlias,
    ) -> Result<reqwest::Response, Error> {
        const _ENDPOINT: &str = "/api/unbound/settings/addHostAlias/";
        let endpoint = self.url(_ENDPOINT);
        let payload: HashMap<&str, &models::NewHostAlias> = [("alias", new)].into_iter().collect();
        let resp = self.client.post(endpoint).json(&payload).send().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SECRET: &str = "SECRET";
    const KEY: &str = "KEY";

    #[tokio::test]
    async fn test_unbound_get_status_running() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundServiceStatus.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "status": "running"
                }
                "#,
            )
            .create();

        let opnsense = Opnsense::new(&host, SECRET, KEY, true).unwrap();

        let status = opnsense.unbound_get_status().await?;
        assert_eq!(status.status, models::StatusType::Running);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_get_status_stopped() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundServiceStatus.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "status": "stopped"
                }
                "#,
            )
            .create();

        let opnsense = Opnsense::new(&host, SECRET, KEY, true).unwrap();

        let status = opnsense.unbound_get_status().await?;
        assert_eq!(status.status, models::StatusType::Stopped);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_get_status_invalid_panics() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundServiceStatus.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "status": "blahblah"
                }
                "#,
            )
            .create();

        let opnsense = Opnsense::new(&host, SECRET, KEY, true).unwrap();

        let status = opnsense.unbound_get_status().await;
        assert!(status.is_err());
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_get_status_extra_keys() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundServiceStatus.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "status": "running",
                    "john": "balatro",
                    "jim": { "bo": true }
                }
                "#,
            )
            .create();

        let opnsense = Opnsense::new(&host, SECRET, KEY, true).unwrap();

        let status = opnsense.unbound_get_status().await?;
        assert_eq!(status.status, models::StatusType::Running);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_get_host_overrides() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundSearchHostOverrides.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "rows": [
                        {
                            "uuid": "some-uuid",
                            "enabled": "1",
                            "hostname": "hostname",
                            "domain": "some-domain",
                            "rr": "A (IPv4 address)",
                            "mxprio": "",
                            "mx": "",
                            "server": "127.0.0.1",
                            "description": "some-description"
                        },
                        {
                            "uuid": "some-uuid2",
                            "enabled": "0",
                            "hostname": "hostname",
                            "domain": "some-domain",
                            "rr": "AAAA (IPv6 address)",
                            "mxprio": "",
                            "mx": "",
                            "server": "192.168.0.1",
                            "description": "some-description2"
                        }
                    ],
                    "rowCount": 2,
                    "total": 2,
                    "current": 1
                }
                "#,
            )
            .create();

        let opnsense = Opnsense::new(&host, SECRET, KEY, true).unwrap();
        let resp = opnsense.unbound_get_host_overrides().await?;

        let expected = models::HostOverride {
            rows: vec![
                models::HostOverrideRow {
                    uuid: "some-uuid".to_string(),
                    enabled: true,
                    hostname: "hostname".to_string(),
                    domain: "some-domain".to_string(),
                    rr: models::HostOverrideType::A,
                    server: "127.0.0.1".to_string(),
                    description: "some-description".to_string(),
                },
                models::HostOverrideRow {
                    uuid: "some-uuid2".to_string(),
                    enabled: false,
                    hostname: "hostname".to_string(),
                    domain: "some-domain".to_string(),
                    rr: models::HostOverrideType::AAAA,
                    server: "192.168.0.1".to_string(),
                    description: "some-description2".to_string(),
                },
            ],
            row_count: 2,
            total: 2,
            current: 1,
        };

        // let status = opnsense.unbonud
        assert_eq!(resp, expected);
        mock.assert();

        Ok(())
    }
}
