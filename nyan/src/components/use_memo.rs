use leptos::prelude::Signal;

pub fn use_memo<T>(f: impl Fn() -> T + Send + Sync + 'static) -> Signal<T>
where
    T: 'static + Send + Sync,
{
    Signal::derive(f)
}
