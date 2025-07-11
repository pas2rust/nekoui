use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    #[default]
    Inherit,
    Italic,
    NotItalic,
}

impl ToStr for FontStyle {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            FontStyle::Inherit => Cow::Borrowed(""),
            FontStyle::Italic => Cow::Borrowed("italic"),
            FontStyle::NotItalic => Cow::Borrowed("not-italic"),
        }
    }
}
