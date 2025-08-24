use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputMessageContainer(
    #[prop(optional, default = Class::new())] class: Class,
    #[prop(optional, default = Class::new())] class_success: Class,
    #[prop(optional, default = Class::new())] class_error: Class,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=use_valid_class(class, class_success, class_error)>
            {children()}
        </p>
    }
}

#[component]
pub fn FormInputMessageSuccess(children: ChildrenFn) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <Show when=move || valid.get() == Valid::Success>
            {children()}
        </Show>
    }
}


#[component]
pub fn FormInputMessageError(children: ChildrenFn) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <Show when=move || valid.get() == Valid::Error>
            {children()}
        </Show>
    }
}

#[component]
pub fn FormInputMessageDefault(children: ChildrenFn) -> impl IntoView {
    let valid = use_input_valid_ctx();
    view! {
        <Show when=move || valid.get() == Valid::Default>
            {children()}
        </Show>
    }
}
