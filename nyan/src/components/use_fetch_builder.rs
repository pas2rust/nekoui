use leptos::prelude::RwSignal;
use neto::components::data::Http;

use crate::components::{data::*, prelude::use_fetch};

pub fn use_fetch_builder<T: Clone + Send + Sync + 'static>(
    f: impl FnOnce(UseFetchParamsBuilder) -> UseFetchParamsBuilder,
) -> Result<(RwSignal<LoadState<T>>, Http), Box<dyn std::error::Error>> {
    let params = f(UseFetchParamsBuilder::default());
    use_fetch(params)
}
