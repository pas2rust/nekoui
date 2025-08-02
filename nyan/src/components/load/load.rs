use crate::components::prelude::*;
use leptos::prelude::*;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Loading,
    Loaded,
}

#[derive(Clone)]
struct LoadState(ReadSignal<State>);

#[component]
pub fn LoadContainer(children: Children) -> impl IntoView {
    let state = use_rw_signal(State::Loading);
    use_provide_ctx(LoadState(state.read_only()));

    use_set_timeout(
        move || {
            state.set(State::Loaded);
        },
        Duration::new(2, 0),
    );

    view! {
        <>
            {children()}
        </>
    }
}

#[component]
pub fn Loading(mut children: ChildrenFnMut) -> impl IntoView {
    let LoadState(state) = use_ctx::<LoadState>().expect("Load state context must be provided");
    view! {
        {move || {
            (state.get() == State::Loading)
                .then(|| view! { <>{children()}</> })
        }}
    }
}

#[component]
pub fn Loaded(mut children: ChildrenFnMut) -> impl IntoView {
    let LoadState(state) = use_ctx::<LoadState>().expect("Load state context must be provided");
    view! {
        {move || {
            (state.get() == State::Loaded)
                .then(|| view! { <>{children()}</> })
        }}
    }
}
