use super::global::GlobalProvider;
use leptos::prelude::*;

#[component]
pub fn NekoノUI・Nyan(
    //#[prop(optional)]
    children: Children,
) -> impl IntoView {
    view! {
        <GlobalProvider>
         {children()}
        </GlobalProvider>
    }
}

#[component]
pub fn NekoProvider(
    //#[prop(optional)]
    children: Children,
) -> impl IntoView {
    view! {
        <GlobalProvider>
         {children()}
        </GlobalProvider>
    }
}
