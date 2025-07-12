use std::time::Duration;

use leptos::prelude::set_timeout;

pub fn use_set_timeout(f: impl FnOnce() + 'static, duration: Duration) {
    set_timeout(f, duration);
}
