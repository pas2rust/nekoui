use leptos::prelude::*;

use crate::components::{
    prelude::{use_effect, use_provide_ctx, use_rw_signal},
    theme::Theme,
    use_ctx::use_ctx,
};

#[component]
pub fn TailwindProvider(children: Children) -> impl IntoView {
    let html_element = document()
        .query_selector("html")
        .expect("query html error")
        .expect("html must be provied");
    let html_data_theme = html_element
        .get_attribute("data-theme")
        .unwrap_or("light".to_string());
    let default_theme = Theme::from_str(&html_data_theme);
    let theme = use_rw_signal(default_theme);
    use_effect(move || {
        let Theme = Theme::from_str(theme.get().to_str());
        let _ = &html_element
            .set_attribute("data-theme", Theme.to_str())
            .expect("set data-theme error");
    });
    use_provide_ctx(theme);

    view! {
        {children()}
    }
}

pub fn use_theme_context() -> RwSignal<Theme> {
    use_ctx::<RwSignal<Theme>>().expect("Theme context not found")
}
