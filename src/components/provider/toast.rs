use crate::components::{adapters::*, toast::types::ToastKind};
use leptos::prelude::*;

pub type Toaster = Vec<ToastKind>;

#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toaster = use_rw_signal(Toaster::new());
    use_provide_ctx(toaster);

    view! {
        <For
            each=move || toaster.get().into_iter().enumerate()
            key=|(index, _)| index.to_string()
            let(item)
        >
            <li>{format!("item: {item:?}")}</li>
        </For>
        {children()}
    }
}

pub fn use_toast_context() -> RwSignal<Toaster> {
    use_ctx::<RwSignal<Toaster>>().expect("Toaster context not found")
}
