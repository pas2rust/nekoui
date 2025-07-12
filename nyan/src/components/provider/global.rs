use super::tailwind::TailwindProvider;
use leptos::prelude::*;

#[component]
pub fn GlobalProvider(children: Children) -> impl IntoView {
    view! {
       <TailwindProvider>

            {children()}

       </TailwindProvider>
    }
}
