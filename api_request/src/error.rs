pub enum Error {
    Reqwest(reqwest::Error),
    Axum(axum::http::Error),
    Serde(serde_json::Error),
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
impl From<axum::http::Error> for Error {
    fn from(value: axum::http::Error) -> Self {
        Self::Axum(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
