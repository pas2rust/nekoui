use kenzu::{Builder, M_Builder};
use leptos::{html::ElementType, prelude::*, reactive::spawn_local};
use neto::components::data::*;
use std::time::Duration;

pub fn use_signal_derive<T: Clone + Send + Sync + 'static>(
    derive: impl Fn() -> T + Send + Sync + 'static,
) -> Signal<T> {
    Signal::derive(derive)
}

pub fn use_node_ref<T: ElementType + 'static>() -> NodeRef<T> {
    NodeRef::new()
}

pub fn use_rw_signal<T: Send + Sync + 'static>(value: T) -> RwSignal<T> {
    RwSignal::new(value)
}

pub fn use_local_rw_signal<T: Send + Sync + 'static>(value: T) -> RwSignal<T, LocalStorage> {
    RwSignal::new_local(value)
}

pub fn use_with_storage_rw_signal<T: Send + Sync + 'static>(value: T) -> RwSignal<T> {
    RwSignal::new_with_storage(value)
}

pub fn use_effect(mut fun: impl FnMut() + 'static) {
    Effect::new(move || fun());
}

pub fn use_ctx<T: Clone + 'static>() -> Option<T> {
    use_context::<T>()
}

pub fn use_provide_ctx<T: Send + Sync + 'static>(value: T) {
    provide_context(value);
}

pub fn use_mount_to_body<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    crate::load_tailwind();
    mount_to_body(f);
}

pub fn use_set_timeout(f: impl FnOnce() + 'static, duration: Duration) {
    set_timeout(f, duration);
}

pub fn use_spawn_local(task: impl Future<Output = ()> + 'static) {
    spawn_local(task);
}

pub fn use_memo<T>(f: impl Fn() -> T + Send + Sync + 'static) -> Signal<T>
where
    T: 'static + Send + Sync,
{
    Signal::derive(f)
}

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

pub fn use_fetch_mut_builder<T: Clone + Send + Sync + 'static>(
    f: impl FnOnce(&mut UseFetchParamsMutBuilder),
) -> Result<(RwSignal<LoadState<T>>, Http), Box<dyn std::error::Error>> {
    let mut params = UseFetchParamsMutBuilder::default();
    f(&mut params);
    use_fetch(params.into())
}

pub fn use_fetch_builder<T: Clone + Send + Sync + 'static>(
    f: impl FnOnce(UseFetchParamsBuilder) -> UseFetchParamsBuilder,
) -> Result<(RwSignal<LoadState<T>>, Http), Box<dyn std::error::Error>> {
    let params = f(UseFetchParamsBuilder::default());
    use_fetch(params)
}
