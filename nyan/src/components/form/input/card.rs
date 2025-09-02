use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FormInputCard(
    children: Children,
    #[prop(optional, default = FormInputCardStyle::class())] class: Class,
) -> impl IntoView {
    view! {
       <span class=class.create()>
            {children()}
       </span>
    }
}
