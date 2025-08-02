use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputContainer(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}
