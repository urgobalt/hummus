use reqwest::header::{CONTENT_TYPE, COOKIE};
use reqwest::{Client, Method};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{JSON_CONTENT_TYPE, RequestBackend, ResponseResult};
/// Using direct communication using [`reqwest`]
pub struct Native;
impl RequestBackend for Native {
    #[inline(always)]
    async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<R> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok(((resp.headers().clone(), resp.status()), resp.json().await?).into())
    }
    #[inline(always)]
    async fn do_string_json_request<const JSON: bool, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<R> {
        let mut resp =
            Client::new().request(method, format!("{base_url}/{url}")).header(COOKIE, cookie);
        if JSON {
            resp = resp.header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        }
        let resp = resp.body(body).send().await?;
        Ok(((resp.headers().clone(), resp.status()), resp.json().await?).into())
    }
    #[inline(always)]
    async fn do_json_status_request<T: Serialize>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<()> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok((resp.headers(), resp.status()).into())
    }

    async fn do_status_request(
        url: &str,
        method: Method,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<()> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .send()
            .await?;
        Ok((resp.headers(), resp.status()).into())
    }

    async fn do_string_string_request<const JSON: bool>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<String> {
        let mut resp =
            Client::new().request(method, format!("{base_url}/{url}")).header(COOKIE, cookie);
        if JSON {
            resp = resp.header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        }
        let resp = resp.body(body).send().await?;
        Ok(((resp.headers().clone(), resp.status()), resp.text().await?).into())
    }
}
