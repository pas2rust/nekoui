use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ToggleContainer(
    #[prop(optional, default = Class::new().light(ToggleContainerStyle::new()))] class: Class,
    children: Children,
) -> impl IntoView {
    let checked = use_rw_signal(false);
    use_provide_ctx(checked);
    view! {
        <label
            class=class.create()
        >
            <input
                type="checkbox"
                class="sr-only"
                prop:checked=move || checked.get()
                on:input=move |_| checked.update(|v| *v = !*v)
            />
            {children()}
        </label>
    }
}

#[component]
pub fn ToggleButton(
    #[prop(optional, default = ToggleStyle::unchecked())] class_unchecked: Class,
    #[prop(optional, default = ToggleStyle::checked())] class_checked: Class,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked = use_context::<RwSignal<bool>>().expect("checked must be provided");
    view! {
        <div
            class=move || if checked.get() {
                class_checked.create()
            } else {
                class_unchecked.create()
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ToggleThumb(
    #[prop(optional, default = Class::new().light(ToggleThumbStyle::md_checked()))]
    class_checked: Class,
    #[prop(optional, default = Class::new().light(ToggleThumbStyle::md_unchecked()))]
    class_unchecked: Class,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked = use_context::<RwSignal<bool>>().expect("ToggleThumb: missing `checked` context");
    view! {
        <div
            class=move || if checked.get() {
                class_checked.create()
            } else {
                class_unchecked.create()
            }
        >
            {children.map(|c| c())}
        </div>
    }
}
