use leptos::prelude::*;

use crate::components::provider::tailwind::TailwindProvider;

#[component]
pub fn GlobalProvider(children: Children) -> impl IntoView {
    view! {
       <TailwindProvider>

            {children()}

       </TailwindProvider>
    }
}
