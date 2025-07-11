use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Position {
    #[default]
    Inherit,
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl ToStr for Position {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Self::Inherit => Cow::Borrowed(""),
            Self::Static => Cow::Borrowed("static"),
            Self::Relative => Cow::Borrowed("relative"),
            Self::Absolute => Cow::Borrowed("absolute"),
            Self::Fixed => Cow::Borrowed("fixed"),
            Self::Sticky => Cow::Borrowed("sticky"),
        }
    }
}
