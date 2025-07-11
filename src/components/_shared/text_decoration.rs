use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    None,
    #[default]
    Inherit,
    Underline,
    LineThrough,
    NoUnderline,
}

impl ToStr for TextDecoration {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            TextDecoration::None => Cow::Borrowed("none"),
            TextDecoration::Inherit => Cow::Borrowed(""),
            TextDecoration::Underline => Cow::Borrowed("underline"),
            TextDecoration::LineThrough => Cow::Borrowed("line-through"),
            TextDecoration::NoUnderline => Cow::Borrowed("no-underline"),
        }
    }
}
