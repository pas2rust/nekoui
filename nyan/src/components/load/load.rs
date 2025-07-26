use std::time::Duration;
use leptos::prelude::*;
use crate::components::prelude::*;

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

    use_set_timeout(move || {
        state.set(State::Loaded);
    }, Duration::new(2, 0));

    view! {
        <div class="load-container">
            {children()}
        </div>
    }
}

#[component]
pub fn Loading(mut children: ChildrenFnMut) -> impl IntoView {
    let LoadState(state) = use_ctx::<LoadState>().expect("Load state context must be provided");
    view! {
        {move || {
            (state.get() == State::Loading)
                .then(|| view! { <div class="text-yellow-500">{children()}</div> })
        }}
    }
}

#[component]
pub fn Loaded(mut children: ChildrenFnMut) -> impl IntoView {
    let LoadState(state) = use_ctx::<LoadState>().expect("Load state context must be provided");
    view! {
        {move || {
            (state.get() == State::Loaded)
                .then(|| view! { <div class="text-green-500">{children()}</div> })
        }}
    }
}
