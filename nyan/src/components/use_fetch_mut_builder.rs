use leptos::prelude::RwSignal;
use neto::components::data::Http;

use crate::components::{data::*, prelude::use_fetch};

pub fn use_fetch_mut_builder<T: Clone + Send + Sync + 'static>(
    f: impl FnOnce(&mut UseFetchParamsMutBuilder),
) -> Result<(RwSignal<LoadState<T>>, Http), Box<dyn std::error::Error>> {
    let mut params = UseFetchParamsMutBuilder::default();
    f(&mut params);
    use_fetch(params.into())
}
