pub mod codegen;
mod datetimerfc3339;
pub use datetimerfc3339::*;
mod auth;
pub use auth::*;

pub(crate) fn pre_hook_mut(inner: &AuthProvider, request: &mut reqwest::Request) {
    inner.sign_request(request);
}

pub(crate) fn post_hook(inner: &AuthProvider, response_result: &Result<reqwest::Response, reqwest::Error>)
{
    dbg!(response_result);
}
