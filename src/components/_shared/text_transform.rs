use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextTransform {
    #[default]
    Inherit,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl ToStr for TextTransform {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            TextTransform::Inherit => Cow::Borrowed(""),
            TextTransform::Uppercase => Cow::Borrowed("uppercase"),
            TextTransform::Lowercase => Cow::Borrowed("lowercase"),
            TextTransform::Capitalize => Cow::Borrowed("capitalize"),
        }
    }
}
