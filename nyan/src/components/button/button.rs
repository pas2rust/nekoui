use super::styles::*;
use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, default = ButtonStyle::neko_neon())] class: Class,
) -> impl IntoView {
    view! {
        <button class=class.create()>
            {children()}
        </button>
    }
}
