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
    #[prop(optional, default = Class::new())] class_error: Class,
) -> impl IntoView {
    let form = use_ctx::<FormData>().expect("Form context not found");
    let debug = use_ctx::<RwSignal<bool>>().expect("debug context value must be provided");
    let local_value = use_rw_signal(String::new());
    let regex = pattern.map(|pat| Regex::new(pat).expect("Invalid regex"));
    let input_valid = use_input_valid_ctx();

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
            input_valid.set(valid);
        } else {
            input_valid.set(Valid::Default)
        }

        form.update(|map| {
            map.insert(name.to_string(), value);
        });
        if debug.get() {
            log!("{:#?}", form.get());
        }
    };

    view! {
        <input
            name=name
            id=name
            placeholder=placeholder
            class=use_valid_class(class, class_success, class_error)
            prop:value=local_value
            on:input=on_input
        />
    }
}
