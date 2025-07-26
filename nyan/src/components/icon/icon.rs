use crate::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Icon(src: Dir) -> impl IntoView {
    let path = src.to_str();
    view! {
        <div
            class="w-8 h-8 bg-current"
            style=format!(
                "\
                mask-image: url('{}'); \
                -webkit-mask-image: url('{}'); \
                mask-repeat: no-repeat; \
                -webkit-mask-repeat: no-repeat; \
                mask-position: center; \
                -webkit-mask-position: center; \
                mask-size: contain; \
                -webkit-mask-size: contain;",
                path, path
            )
            role="img"
            aria-label="icon"
        />
    }
}
