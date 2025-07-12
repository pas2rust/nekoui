use leptos::{IntoView, mount::mount_to_body};

pub fn use_mount_to_body<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    mount_to_body(f);
}
