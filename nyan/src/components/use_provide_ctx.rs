use leptos::prelude::provide_context;

pub fn use_provide_ctx<T: Send + Sync + 'static>(value: T) {
    provide_context(value);
}
