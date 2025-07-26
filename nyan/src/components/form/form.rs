use crate::components::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;
use neto::regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Valid {
    Ok,
    Err,
    None,
}

type FormData = RwSignal<HashMap<String, String>>;
type FormValid = RwSignal<Valid>;

#[component]
pub fn FormContainer(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    let form = use_rw_signal::<HashMap<String, String>>(HashMap::new());
    use_provide_ctx::<FormData>(form);
    let is_valid = use_rw_signal(Valid::None);
    use_provide_ctx::<FormValid>(is_valid);
    view! {
        <form class=class.create()>
            {children()}
        </form>
    }
}

#[component]
pub fn FormInputContainer(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}

#[component]
pub fn FormInputLabel(
    children: Children,
    #[prop(optional, default = Class::new())] class_success: Class,
    #[prop(optional, default = Class::new())] class_error: Class,
) -> impl IntoView {
    view! {
        <label for="error" class=class_success.create()>
            {children()}
        </label>
    }
}

#[component]
pub fn FormInputMessage(
    children: Children,
    #[prop(optional, default = Class::new())] class: Class,
) -> impl IntoView {
    view! {
        <p class="mt-2 text-sm text-red-600 dark:text-red-500">
            <span class="font-medium">Oh, snapp!</span> {children()}
        </p>
    }
}

#[component]
pub fn FormInput(
    name: &'static str,
    placeholder: &'static str,
    #[prop(optional)] pattern: Option<&'static str>,
) -> impl IntoView {
    let form = use_ctx::<FormData>().expect("Form context not found");
    let local_value = use_rw_signal(String::new());
    let is_valid = use_ctx::<FormValid>().expect("Form valid context not found");

    let regex = pattern.map(|pat| Regex::new(pat).expect("Invalid regex"));

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        local_value.set(value.clone());

        let valid = match &regex {
            Some(re) => re.is_match(&value),
            None => true,
        };

        let valid = match valid {
            true => Valid::Ok,
            false => Valid::Err,
        };

        if !value.is_empty() {
            is_valid.set(valid);
        } else {
            is_valid.set(Valid::None)
        }

        if valid == Valid::Ok {
            form.update(|map| {
                map.insert(name.to_string(), value);
            });
            log!("{:#?}", form.get());
        }
    };

    view! {
        <input
            name=name
            id=name
            placeholder=placeholder
            class="border p-2 rounded"
            prop:value=local_value
            on:input=on_input
        />
    }
}
