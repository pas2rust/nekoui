use crate::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Valid {
    Success,
    Error,
    None,
}
pub type FormValid = RwSignal<Valid>;

#[component]
pub fn FormInputContainer(
    children: Children,
    #[prop(optional, default = FormInputContainerStyle::class())] 
    class: Class
) -> impl IntoView {
    let is_valid = use_rw_signal(Valid::None);
    use_provide_ctx::<FormValid>(is_valid);
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}
