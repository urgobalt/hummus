use super::{JSON_CONTENT_TYPE, RequestBackend};
use crate::error::Error;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, StatusCode, header::COOKIE};
use serde::{Serialize, de::DeserializeOwned};

pub struct Native;
impl RequestBackend for Native {
    #[inline(always)]
    async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }
    #[inline(always)]
    async fn do_string_json_request<const JSON: bool, R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        let mut resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie);
        if JSON {
            resp = resp.header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        }
        let resp = resp.body(body).send().await?;
        Ok((resp.status(), resp.json().await?))
    }
    #[inline(always)]
    async fn do_json_status_request<T: Serialize>(
        url: &'static str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<StatusCode, Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok(resp.status())
    }

    async fn do_status_request(
        url: &'static str,
        method: Method,
        base_url: &str,
        cookie: &str,
    ) -> Result<StatusCode, Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .send()
            .await?;
        Ok(resp.status())
    }

    async fn do_string_string_request<const JSON: bool>(
        url: &'static str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, String), Error> {
        let mut resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie);
        if JSON {
            resp = resp.header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        }
        let resp = resp.body(body).send().await?;
        Ok((resp.status(), resp.text().await?))
    }
}
