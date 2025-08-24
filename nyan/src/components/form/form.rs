use crate::components::prelude::*;
use leptos::prelude::*;
use std::collections::HashMap;

pub type FormData = RwSignal<HashMap<String, String>>;


#[component]
pub fn FormContainer(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    let form = use_rw_signal::<HashMap<String, String>>(HashMap::new());
    use_provide_ctx::<FormData>(form);
    view! {
        <form class=class.create()>
            {children()}
        </form>
    }
}