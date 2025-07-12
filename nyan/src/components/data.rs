use kenzu::{Builder, M_Builder};
use neto::components::data::*;

#[derive(Clone)]
pub enum LoadState<T: Clone> {
    Loading,
    Success(T),
    Error(String),
}

#[derive(Debug, Builder, Default)]
pub struct UseFetchParamsBuilder {
    pub base_url: String,
    pub token: String,
    pub headers: Header,
}

#[derive(Debug, M_Builder, Default)]
pub struct UseFetchParamsMutBuilder {
    pub base_url: String,
    pub token: String,
    pub headers: Header,
}

impl From<UseFetchParamsMutBuilder> for UseFetchParamsBuilder {
    fn from(m: UseFetchParamsMutBuilder) -> Self {
        Self {
            base_url: m.base_url,
            token: m.token,
            headers: m.headers,
        }
    }
}
