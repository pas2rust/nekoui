use leptos::prelude::Signal;

pub fn use_signal_derive<T: Clone + Send + Sync + 'static>(
    derive: impl Fn() -> T + Send + Sync + 'static,
) -> Signal<T> {
    Signal::derive(derive)
}
