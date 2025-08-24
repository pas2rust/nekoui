use crate::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Valid {
    Success,
    Error,
    Default,
}
pub type InputValid = RwSignal<Valid>;

#[component]
pub fn FormInputContainer(
    children: Children,
    #[prop(optional, default = FormInputContainerStyle::class())] 
    class: Class
) -> impl IntoView {
    let is_valid = use_rw_signal(Valid::Default);
    use_provide_ctx::<InputValid>(is_valid);
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}
