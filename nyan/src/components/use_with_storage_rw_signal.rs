use leptos::prelude::RwSignal;

pub fn use_with_storage_rw_signal<T: Send + Sync + 'static>(value: T) -> RwSignal<T> {
    RwSignal::new_with_storage(value)
}
