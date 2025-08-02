use crate::components::prelude::*;
use leptos::prelude::*;


#[component]
pub fn FormInputLabel(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
    #[prop(optional, default = Class::new())] class_success: Class,
    #[prop(optional, default = Class::new())] class_erroror: Class,
) -> impl IntoView {
    let form_valid = use_ctx::<FormValid>().expect("Form valid must be provided!");
    let class = move || {
        match form_valid.get() {
            Valid::None => class.create(),
            Valid::Success => class_success.create(),
            Valid::Error => class_erroror.create(),
        }
        .get()
    };
    view! {
        <label for="" class=class>
            {children()}
        </label>
    }
}
