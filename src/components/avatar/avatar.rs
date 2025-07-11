use super::prelude::*;
use crate::components::{
    _shared::prelude::*,
    adapters::{use_provide_ctx, use_rw_signal},
};
use leptos::prelude::*;

#[component]
pub fn AvatarImage(
    src: Dir,
    #[prop(optional, default = Class::new().light(AvatarStyle::xs()))] class: Class,
) -> impl IntoView {
    view! {
        <img class=class.create() src=src.to_str() />
    }
}

#[component]
pub fn AvatarContainer(
    #[prop(optional, default = Class::new().light(AvatarContainerStyle::new()))] class: Class,
    children: Children,
) -> impl IntoView {
    let hover = use_rw_signal(false);
    use_provide_ctx(hover);
    view! {
        <div
            class=class.create()
            on:mouseenter=move |_| hover.set(true)
            on:mouseleave=move |_| hover.set(false)
        >
           {children()}
        </div>
    }
}

#[component]
pub fn AvatarDot(
    #[prop(optional, default = Class::new().light(AvatarDotStyle::xs()))] class: Class,
) -> impl IntoView {
    let hover = use_context::<RwSignal<bool>>().expect("hover context must be provided");
    view! {
        <div
            class=class.create()
            class:animate-bounce=move || hover.get()
        />
    }
}
