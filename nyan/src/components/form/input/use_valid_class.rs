use leptos::prelude::Get;

use crate::prelude::{use_ctx, Class, FormValid, ToClass, Valid};

pub fn use_valid_class(
    class: Class,
    class_success: Class,
    class_error: Class
) -> impl Fn() -> String {
    let form_valid = use_ctx::<FormValid>().expect("Form valid must be provided!");
    move || {
        match form_valid.get() {
            Valid::None => class.clone(),
            Valid::Success => class_success.clone(),
            Valid::Error => class_error.clone(),
        }.to_class()
    }
}
