use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputMessage(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
    #[prop(optional, default = Class::new())] class_success: Class,
    #[prop(optional, default = Class::new())] class_error: Class,
) -> impl IntoView {
    view! {
        <p class=use_valid_class(class, class_success, class_error)>
            <span class="font-medium">Oh, snapp!</span> {children()}
        </p>
    }
}
