use super::styles::ButtonStyle;
use crate::components::_shared::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, default = Class::new()
        .light(ButtonStyle::glass())
        .dark(ButtonStyle::neon()))
    ]
    class: Class,
) -> impl IntoView {
    view! {
        <button class=class.create()>
            {children()}
        </button>
    }
}
