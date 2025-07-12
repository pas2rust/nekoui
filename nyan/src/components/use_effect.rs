use leptos::prelude::Effect;

pub fn use_effect(mut fun: impl FnMut() + 'static) {
    Effect::new(move || fun());
}
