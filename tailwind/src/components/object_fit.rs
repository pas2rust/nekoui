use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ObjectFit {
    #[default]
    Inherit,
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

impl ToStr for ObjectFit {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Self::Inherit => Cow::Borrowed(""),
            Self::Fill => Cow::Borrowed("fill"),
            Self::Contain => Cow::Borrowed("contain"),
            Self::Cover => Cow::Borrowed("cover"),
            Self::None => Cow::Borrowed("none"),
            Self::ScaleDown => Cow::Borrowed("scale-down"),
        }
    }
}
