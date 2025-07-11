use std::time::Duration;

use leptos::prelude::{Get, RwSignal, Set, Signal, set_timeout};

use crate::components::adapters::*;

pub type LoadSignal<T> = (RwSignal<LoadState<T>>, RwSignal<bool>, Signal<bool>);

#[derive(Clone)]
pub enum LoadState<T> {
    Loading,
    Success(T),
    Error(String),
}

pub fn use_load<T: 'static + Send + Sync>(
    initial: LoadState<T>,
    duration: Option<(u64, u32)>,
) -> LoadSignal<T> {
    let result = use_rw_signal(initial);
    let loading = use_rw_signal(true);
    let waiting = use_rw_signal(true);
    let is_loading = Signal::derive(move || waiting.get() || loading.get());

    let (secs, nanos) = duration.unwrap_or((2, 500));

    use_effect(move || {
        if loading.get() {
            waiting.set(true);

            set_timeout(move || waiting.set(false), Duration::new(secs, nanos));
        }
    });

    (result, loading, is_loading)
}
