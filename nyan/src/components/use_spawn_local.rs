use leptos::reactive::spawn_local;

pub fn use_spawn_local(task: impl Future<Output = ()> + 'static) {
    spawn_local(task);
}
