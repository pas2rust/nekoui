use leptos::prelude::RwSignal;

pub fn use_rw_signal<T: Send + Sync + 'static>(value: T) -> RwSignal<T> {
    RwSignal::new(value)
}
