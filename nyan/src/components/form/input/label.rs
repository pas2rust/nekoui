use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputLabel(
    children: Children,
    #[prop(optional, default = FormInputLabelStyle::class())] class: Class,
    #[prop(optional, default = FormInputLabelStyle::class_success())] class_success: Class,
    #[prop(optional, default = FormInputLabelStyle::class_error())] class_error: Class,
) -> impl IntoView {
    view! {
        <label class=use_valid_class(class, class_success, class_error)>
            {children()}
        </label>
    }
}
