use leptos::prelude::RwSignal;
use neto::{
    components::data::Http,
    reqwest::header::{HeaderName, HeaderValue},
};

use crate::components::{data::*, prelude::use_rw_signal};

pub fn use_fetch<T: Clone + Send + Sync + 'static>(
    params: UseFetchParamsBuilder,
) -> Result<(RwSignal<LoadState<T>>, Http), Box<dyn std::error::Error>> {
    let UseFetchParamsBuilder {
        base_url,
        mut headers,
        token,
    } = params;
    let result = use_rw_signal(LoadState::Loading);
    headers.extend([(
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&token).expect("From str value in token failed"),
    )]);

    let mut fetch = Http::new().base_url(base_url).headers(headers).build()?;

    fetch.config()?;

    Ok((result, fetch))
}
