use axum::http::{HeaderMap, Request as AxumRequest, Response as AxumResponse};
pub const SET_BASE: &str = "SET_BASE";
use reqwest::header::COOKIE;
use reqwest::{Body, Client, Method, StatusCode, Upgraded, Version};
use reqwest::{Request as ReqwestRequest, Response as ReqwestResponse};
pub struct Request<'a, T> {
    path_uri: &'static str,
    base: &'a str,
    method: Method,
    cookie: String,
    body: T,
}
impl<'a, T> Into<AxumRequest<T>> for Request<'a, T> {
    fn into(self) -> AxumRequest<T> {
        AxumRequest::builder()
            .uri(format!("{}", self.path_uri))
            .method(self.method)
            .header(SET_BASE, self.base)
            .header(COOKIE, self.cookie)
            .version(Version::HTTP_2)
            .body(self.body)
            .expect("Invalid request")
    }
}
impl<'a, T: Into<Body>> Into<ReqwestRequest> for Request<'a, T> {
    fn into(self) -> ReqwestRequest {
        Client::new()
            .request(self.method, format!("{}/{}", self.base, self.path_uri))
            .header(COOKIE, self.cookie)
            .version(Version::HTTP_2)
            .header(SET_BASE, self.base)
            .body(self.body)
            .build()
            .expect("Invalid Request construction")
    }
}
pub struct Response<T> {
    body: T,
    statuscode: StatusCode,
    headers: HeaderMap,
}
impl<T> From<ReqwestResponse> for Response<T> {
    fn from(value: ReqwestResponse) -> Self {
        Self {
            statuscode: value.status(),
            headers: value.headers().clone(),
            body: futures::executor::block_on(value.text()).expect(),
        }
    }
}
impl From<AxumResponse> for Response {
    fn from(value: AxumResponse) -> Self {
        Self {
            statuscode: value.status(),
            body: value.body(),
            headers: value.headers().clone(),
        }
    }
}
