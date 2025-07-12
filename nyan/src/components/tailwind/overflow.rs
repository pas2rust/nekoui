use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Overflow {
    #[default]
    Inherit,
    None,
    Hidden,
    Scroll,
    Auto,
}

impl ToStr for Overflow {
    fn to_str(&self) -> Cow<'static, str> {
        let s = match self {
            Overflow::Inherit => "",
            Overflow::None => "none",
            Overflow::Hidden => "hidden",
            Overflow::Scroll => "scroll",
            Overflow::Auto => "auto",
        };
        Cow::Borrowed(s)
    }
}
