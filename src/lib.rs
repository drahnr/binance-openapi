pub mod codegen;
mod datetimerfc3339;
pub use datetimerfc3339::*;


#[derive(Debug, Clone)]
pub struct AuthProvider {
    pub api_key: String,
}

impl AuthProvider {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let api_key = std::env::var("SEVDESK_TOKEN")?;
        Ok(Self { api_key })
    }

    pub fn header_api_key(&self, _header_name: impl AsRef<str>) -> String {
        self.api_key.clone()
    }
}


pub(crate) fn pre_hook(inner: &AuthProvider, request: &reqwest::Request) {
    dbg!(request);
}

pub(crate) fn post_hook(inner: &AuthProvider, response_result: &Result<reqwest::Response, reqwest::Error>)
{
    dbg!(response_result);
}
