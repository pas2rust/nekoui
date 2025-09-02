use leptos::prelude::Get;

use crate::prelude::*;

pub fn use_valid_class(
    class: Class,
    class_success: Class,
    class_error: Class,
) -> impl Fn() -> String {
    let form_valid = use_input_valid_ctx();
    move || {
        match form_valid.get() {
            Valid::Default => class.clone(),
            Valid::Success => class_success.clone(),
            Valid::Error => class_error.clone(),
        }
        .to_class()
    }
}
