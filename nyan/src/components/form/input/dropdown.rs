use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputDropdownMenu(
    children: Children,
    #[prop(optional, default = FormInputContainerStyle::class())] class: Class,
) -> impl IntoView {
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}

#[component]
pub fn FormInputDropdownDivide(
    children: Children,
    #[prop(optional, default = FormInputContainerStyle::class())] class: Class,
) -> impl IntoView {
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}

#[component]
pub fn FormInputDropdownItem(
    children: Children,
    #[prop(optional, default = FormInputContainerStyle::class())] class: Class,
) -> impl IntoView {
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}
