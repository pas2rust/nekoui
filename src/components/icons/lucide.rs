use path2enum::project_paths_enum;
use leptos::prelude::*;

#[project_paths_enum(path = "client/nekoui/public/assets/lucide", ext = "svg")]
pub enum Lucide {}

#[component]
pub fn Lucide(src: Lucide) -> impl IntoView {
    let path = format!("client/nekoui/public/assets/lucide/{}.svg", src.to_str());
    view! {
        <img src=path alt=src.to_str()/>
    }
}

pub enum A {
    DirノSubdir
}