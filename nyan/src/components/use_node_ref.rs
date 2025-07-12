use leptos::{html::ElementType, prelude::NodeRef};

pub fn use_node_ref<T: ElementType + 'static>() -> NodeRef<T> {
    NodeRef::new()
}
