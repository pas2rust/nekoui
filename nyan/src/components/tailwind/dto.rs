use std::borrow::Cow;

pub trait ToStr {
    fn to_str(&self) -> Cow<'static, str>;
}

pub trait ToClass {
    fn to_class(&self) -> String;
}

pub trait BuildClass {
    fn build() -> Vec<Box<dyn ToClass>>;
}
