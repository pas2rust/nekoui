use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputMessageSuccess(
    #[prop(optional, default = Class::new())] class: Class,
    children: ChildrenFn,
) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <p class=class.create()>
            <Show when=move || valid.get() == Valid::Success>
                {children()}
            </Show>
        </p>
    }
}

#[component]
pub fn FormInputMessageError(
    #[prop(optional, default = Class::new())] class: Class,
    children: ChildrenFn,
) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <p class=class.create()>
            <Show when=move || valid.get() == Valid::Error>
                {children()}
            </Show>
        </p>
    }
}

#[component]
pub fn FormInputMessageDefault(
    #[prop(optional, default = Class::new())] class: Class,
    children: ChildrenFn,
) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <p class=class.create()>
            <Show when=move || valid.get() == Valid::Default>
                {children()}
            </Show>
        </p>
    }
}
