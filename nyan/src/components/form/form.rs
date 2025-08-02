use crate::components::prelude::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Valid {
    Success,
    Error,
    None,
}

pub type FormData = RwSignal<HashMap<String, String>>;
pub type FormValid = RwSignal<Valid>;

#[component]
pub fn FormContainer(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    let form = use_rw_signal::<HashMap<String, String>>(HashMap::new());
    use_provide_ctx::<FormData>(form);
    let is_valid = use_rw_signal(Valid::None);
    use_provide_ctx::<FormValid>(is_valid);
    view! {
        <form class=class.create()>
            {children()}
        </form>
    }
}