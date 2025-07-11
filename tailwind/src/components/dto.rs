use std::borrow::Cow;

pub trait ToStr {
    fn to_str(&self) -> Cow<'static, str>;
}
