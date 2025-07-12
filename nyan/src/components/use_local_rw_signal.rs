use leptos::prelude::{LocalStorage, RwSignal};

pub fn use_local_rw_signal<T: 'static>(value: T) -> RwSignal<T, LocalStorage> {
    RwSignal::new_local(value)
}
