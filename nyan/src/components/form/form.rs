use crate::components::prelude::*;
use leptos::prelude::*;
use std::collections::HashMap;

pub type FormData = RwSignal<HashMap<String, String>>;

#[component]
pub fn FormContainer(
    children: Children,
    #[prop(optional, default = false)] debug: bool,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    let form = use_rw_signal::<HashMap<String, String>>(HashMap::new());
    let debug = use_rw_signal(debug);
    use_provide_ctx::<FormData>(form);
    use_provide_ctx(debug);
    view! {
        <form class=class.create()>
            {children()}
        </form>
    }
}
