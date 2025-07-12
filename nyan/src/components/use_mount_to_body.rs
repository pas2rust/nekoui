use leptos::{IntoView, mount::mount_to_body};

pub fn use_mount_to_body<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    use wasm_bindgen::JsCast;
    use web_sys::{Document, HtmlLinkElement, Window};

    let window: Window = web_sys::window().expect("no window");
    let document: Document = window.document().expect("no document");

    let link = document
        .create_element("link")
        .expect("create link")
        .dyn_into::<HtmlLinkElement>()
        .expect("cast to HtmlLinkElement");

    link.set_rel("stylesheet");
    link.set_href("public/tailwind.css");

    document
        .head()
        .expect("no head")
        .append_child(&link)
        .expect("failed to append link");
    mount_to_body(f);
}
