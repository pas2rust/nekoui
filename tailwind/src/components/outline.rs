use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Outline {
    #[default]
    Inherit,
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

impl ToStr for Outline {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Outline::Inherit => Cow::Borrowed(""),
            Outline::None => Cow::Borrowed("none"),
            Outline::Solid => Cow::Borrowed(""),
            Outline::Dashed => Cow::Borrowed("dashed"),
            Outline::Dotted => Cow::Borrowed("dotted"),
            Outline::Double => Cow::Borrowed("double"),
        }
    }
}
