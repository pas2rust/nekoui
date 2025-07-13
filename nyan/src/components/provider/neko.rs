use leptos::prelude::*;
use super::global::GlobalProvider;

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