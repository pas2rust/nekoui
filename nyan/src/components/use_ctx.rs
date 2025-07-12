use leptos::prelude::use_context;

pub fn use_ctx<T: Clone + 'static>() -> Option<T> {
    use_context::<T>()
}
