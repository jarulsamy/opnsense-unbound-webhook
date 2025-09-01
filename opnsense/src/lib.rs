pub mod models;

use std::collections::HashMap;

use anyhow::{Context, Error, Result, anyhow};
use base64::{Engine, engine::general_purpose};
use reqwest;
use reqwest::header;

pub struct Opnsense {
    pub url: String,

    client: reqwest::Client,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ApiEndpoint {
    UnboundServiceStatus,
    UnboundSearchHostOverrides,
    UnboundAddHostOverride,
    UnboundDelHostOverride,
    UnboundSearchHostAliases,
    UnboundAddHostAlias,
    UnboundDelHostAlias,
}

impl From<ApiEndpoint> for &'static str {
    fn from(endpoint: ApiEndpoint) -> Self {
        match endpoint {
            ApiEndpoint::UnboundServiceStatus => "/api/unbound/service/status",
            ApiEndpoint::UnboundSearchHostOverrides => "/api/unbound/settings/searchHostOverride/",
            ApiEndpoint::UnboundAddHostOverride => "/api/unbound/settings/addHostOverride/",
            ApiEndpoint::UnboundDelHostOverride => "/api/unbound/settings/delHostOverride/",
            ApiEndpoint::UnboundSearchHostAliases => "/api/unbound/settings/searchHostAlias/",
            ApiEndpoint::UnboundAddHostAlias => "/api/unbound/settings/addHostAlias/",
            ApiEndpoint::UnboundDelHostAlias => "/api/unbound/settings/delHostAlias/",
        }
    }
}

impl Opnsense {
    pub fn new(
        url: &str,
        key: Option<String>,
        secret: Option<String>,
        danger_accept_invalid_certs: bool,
    ) -> Result<Self, Error> {
        let key = key.unwrap_or_default();
        let secret = secret.unwrap_or_default();

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
        &self,
        new: &models::NewHostOverride,
    ) -> Result<models::Uuid, Error> {
        let endpoint: &str = ApiEndpoint::UnboundAddHostOverride.into();
        let url = self.url(endpoint);
        let payload: HashMap<&str, &models::NewHostOverride> =
            [("host", new)].into_iter().collect();
        let resp = self.client.post(url).json(&payload).send().await?;
        let resp = resp.error_for_status()?;
        let parsed = resp.json::<models::ApiResult>().await?;

        if parsed.result == "failed" {
            Err(anyhow!(format!(
                "Operation failed: {:?}",
                parsed.validations
            )))?
        }

        parsed.uuid.ok_or(anyhow!("Failed to parse UUID"))
    }

    pub async fn unbound_del_host_override(&self, uuid: String) -> Result<(), Error> {
        let endpoint: &str = ApiEndpoint::UnboundDelHostOverride.into();
        let url = self.url(endpoint) + &uuid;
        let resp = self.client.post(&url).body("{}").send().await?;
        let parsed = resp.json::<models::ApiResult>().await?;
        if parsed.result != "deleted" {
            Err(anyhow!(format!(
                "Operation failed: {:?}",
                parsed.validations
            )))?
        }

        Ok(())
    }

    pub async fn unbound_get_host_aliases(&self) -> Result<models::HostAlias, Error> {
        let endpoint: &str = ApiEndpoint::UnboundSearchHostAliases.into();
        let url = self.url(endpoint);
        let resp = self.client.get(url).send().await?;
        let parsed = resp.json::<models::HostAlias>().await?;
        Ok(parsed)
    }

    pub async fn unbound_add_host_alias(
        &self,
        new: &models::NewHostAlias,
    ) -> Result<models::Uuid, Error> {
        let endpoint: &str = ApiEndpoint::UnboundAddHostAlias.into();
        let url = self.url(endpoint);
        let payload: HashMap<&str, &models::NewHostAlias> = [("alias", new)].into_iter().collect();
        let resp = self.client.post(url).json(&payload).send().await?;
        let resp = resp.error_for_status()?;
        let parsed = resp.json::<models::ApiResult>().await?;

        if parsed.result == "failed" {
            Err(anyhow!(format!(
                "Operation failed: {:?}",
                parsed.validations
            )))?
        }

        parsed.uuid.ok_or(anyhow!("Failed to parse UUID"))
    }

    pub async fn unbound_del_host_alias(&self, uuid: String) -> Result<(), Error> {
        let endpoint: &str = ApiEndpoint::UnboundDelHostAlias.into();
        let url = self.url(endpoint) + &uuid;
        let resp = self.client.post(&url).body("{}").send().await?;
        let parsed = resp.json::<models::ApiResult>().await?;
        if parsed.result != "deleted" {
            Err(anyhow!(format!(
                "Operation failed: {:?}",
                parsed.validations
            )))?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Matcher;
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

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();

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

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();

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

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();

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

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();

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

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
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

        assert_eq!(resp, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_add_host_override() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        let expected = r#"
            {
                "host": {
                    "enabled": "1",
                    "hostname": "hostname",
                    "domain": "domain",
                    "rr": "A",
                    "mxprio": "",
                    "mx": "",
                    "server": "server",
                    "description": "description"
                }
            }
        "#;

        // // Create a mock
        let mock = server
            .mock::<&str>("POST", ApiEndpoint::UnboundAddHostOverride.into())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "saved",
                    "uuid": "some-uuid"
                }
                "#,
            )
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let payload = models::NewHostOverride {
            enabled: true,
            hostname: "hostname".to_string(),
            domain: "domain".to_string(),
            rr: models::HostOverrideType::A,
            mxprio: "".to_string(),
            mx: "".to_string(),
            server: "server".to_string(),
            description: "description".to_string(),
        };
        opnsense.unbound_add_host_override(&payload).await?;

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_add_host_override_failed() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        let expected = r#"
            {
                "host": {
                    "enabled": "1",
                    "hostname": "hostname",
                    "domain": "domain",
                    "rr": "A",
                    "mxprio": "",
                    "mx": "",
                    "server": "server",
                    "description": "description"
                }
            }
        "#;

        // // Create a mock
        let mock = server
            .mock::<&str>("POST", ApiEndpoint::UnboundAddHostOverride.into())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "failed",
                    "validations": {
                        "reason": "unknown"
                    }
                }
                "#,
            )
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let payload = models::NewHostOverride {
            enabled: true,
            hostname: "hostname".to_string(),
            domain: "domain".to_string(),
            rr: models::HostOverrideType::A,
            mxprio: "".to_string(),
            mx: "".to_string(),
            server: "server".to_string(),
            description: "description".to_string(),
        };
        let resp = opnsense.unbound_add_host_override(&payload).await;

        mock.assert();
        assert!(resp.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_del_host_override() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);
        let uuid = "someuuid";

        let expected = r#"{}"#;
        let endpoint = <ApiEndpoint as Into<&str>>::into(ApiEndpoint::UnboundDelHostOverride)
            .to_string()
            + uuid;

        let mock = server
            .mock::<&str>("POST", &endpoint)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "deleted"
                }
                "#,
            )
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let resp = opnsense.unbound_del_host_override(uuid.to_string()).await?;

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_del_host_override_missing() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);
        let uuid = "someuuid";

        let expected = r#"{}"#;
        let endpoint = <ApiEndpoint as Into<&str>>::into(ApiEndpoint::UnboundDelHostOverride)
            .to_string()
            + uuid;

        let mock = server
            .mock::<&str>("POST", &endpoint)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "not found"
                }
                "#,
            )
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let resp = opnsense.unbound_del_host_override(uuid.to_string()).await;

        mock.assert();
        assert!(resp.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_get_host_aliases() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        // // Create a mock
        let mock = server
            .mock::<&str>("GET", ApiEndpoint::UnboundSearchHostAliases.into())
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "rows": [
                        {
                            "uuid": "some-uuid",
                            "enabled": "1",
                            "host": "some-host",
                            "hostname": "some-hostname",
                            "domain": "some-domain",
                            "description": "some-description"
                        }
                    ],
                    "rowCount": 1,
                    "total": 1,
                    "current": 1
                }
                "#,
            )
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let resp = opnsense.unbound_get_host_aliases().await?;

        let expected = models::HostAlias {
            rows: vec![models::HostAliasRow {
                uuid: "some-uuid".to_string(),
                enabled: true,
                host: "some-host".to_string(),
                hostname: "some-hostname".to_string(),
                domain: "some-domain".to_string(),
                description: "some-description".to_string(),
            }],
            row_count: 1,
            total: 1,
            current: 1,
        };

        // let status = opnsense.unbonud
        assert_eq!(resp, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_add_host_alias() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        let expected = r#"
            {
                "alias": {
                    "description": "some-description",
                    "domain": "some-domain",
                    "enabled": "1",
                    "hostname": "some-hostname",
                    "host": "some-host-uuid"
                }
            }
        "#;

        // // Create a mock
        let mock = server
            .mock::<&str>("POST", ApiEndpoint::UnboundAddHostAlias.into())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "saved",
                    "uuid": "some-uuid"
                }
                "#,
            )
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let payload = models::NewHostAlias {
            description: "some-description".to_string(),
            domain: "some-domain".to_string(),
            enabled: true,
            host: "some-host-uuid".to_string(),
            hostname: "some-hostname".to_string(),
        };

        let uuid = opnsense.unbound_add_host_alias(&payload).await?;

        mock.assert();
        assert_eq!(uuid, "some-uuid");

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_add_host_alias_invalid_host() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);

        let expected = r#"
            {
                "alias": {
                    "description": "some-description",
                    "domain": "some-domain",
                    "enabled": "1",
                    "hostname": "some-hostname",
                    "host": "a-nonexistent-uuid"
                }
            }
        "#;

        // // Create a mock
        let mock = server
            .mock::<&str>("POST", ApiEndpoint::UnboundAddHostAlias.into())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "failed",
                    "validations": {
                        "alias.host": "Option not in this list."
                    }
                }
                "#,
            )
            .match_header("content-type", "application/json")
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let payload = models::NewHostAlias {
            description: "some-description".to_string(),
            domain: "some-domain".to_string(),
            enabled: true,
            host: "a-nonexistent-uuid".to_string(),
            hostname: "some-hostname".to_string(),
        };
        let resp = opnsense.unbound_add_host_alias(&payload).await;

        mock.assert();
        assert!(resp.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_del_host_alias() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);
        let uuid = "someuuid";

        let expected = r#"{}"#;
        let endpoint =
            <ApiEndpoint as Into<&str>>::into(ApiEndpoint::UnboundDelHostAlias).to_string() + uuid;

        let mock = server
            .mock::<&str>("POST", &endpoint)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "deleted"
                }
                "#,
            )
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let resp = opnsense.unbound_del_host_alias(uuid.to_string()).await?;

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_unbound_del_host_alias_missing() -> Result<(), Error> {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let host = format!("http://{}", host);
        let uuid = "someuuid";

        let expected = r#"{}"#;
        let endpoint =
            <ApiEndpoint as Into<&str>>::into(ApiEndpoint::UnboundDelHostAlias).to_string() + uuid;

        let mock = server
            .mock::<&str>("POST", &endpoint)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("accept", "application/json")
            .with_body(
                r#"
                {
                    "result": "not found"
                }
                "#,
            )
            .match_header("accept", "application/json")
            .match_body(Matcher::JsonString(expected.to_string()))
            .create();

        let opnsense =
            Opnsense::new(&host, Some(SECRET.to_string()), Some(KEY.to_string()), true).unwrap();
        let resp = opnsense.unbound_del_host_alias(uuid.to_string()).await;

        mock.assert();
        assert!(resp.is_err());

        Ok(())
    }
}
