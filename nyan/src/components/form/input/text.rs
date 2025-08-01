use crate::components::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;
use neto::regex::Regex;

#[component]
pub fn FormInputText(
    name: &'static str,
    placeholder: &'static str,
    #[prop(optional)] pattern: Option<&'static str>,
    #[prop(optional, default = Class::new())] class: Class,
    #[prop(optional, default = Class::new())] class_success: Class,
    #[prop(optional, default = Class::new())] class_erroror: Class,
    #[prop(optional, default = false)] debug: bool,
) -> impl IntoView {
    let form = use_ctx::<FormData>().expect("Form context not found");
    let local_value = use_rw_signal(String::new());
    let regex = pattern.map(|pat| Regex::new(pat).expect("Invalid regex"));
    let form_valid = use_ctx::<FormValid>().expect("Form valid must be provided!");
    let class = move || {
        match form_valid.get() {
            Valid::None => class.create(),
            Valid::Success => class_success.create(),
            Valid::Error => class_erroror.create(),
        }
        .get()
    };

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        local_value.set(value.clone());

        let valid = match &regex {
            Some(re) => re.is_match(&value),
            None => true,
        };

        let valid = match valid {
            true => Valid::Success,
            false => Valid::Error,
        };

        if !value.is_empty() {
            form_valid.set(valid);
        } else {
            form_valid.set(Valid::None)
        }

        if valid == Valid::Success {
            form.update(|map| {
                map.insert(name.to_string(), value);
            });
            if debug {
                log!("{:#?}", form.get());
            }
        }
    };

    view! {
        <input
            name=name
            id=name
            placeholder=placeholder
            class=class
            prop:value=local_value
            on:input=on_input
        />
    }
}