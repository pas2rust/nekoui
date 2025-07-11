use super::dto::ToStr;
use std::borrow::Cow;


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Transition {
    #[default]
    Inherit,
    All,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Custom(&'static str),
}

impl ToStr for Transition {
    fn to_str(&self) -> Cow<'static, str> {
        use Transition::*;
        Cow::Borrowed(match self {
            Inherit => "",
            All => "all",
            Colors => "colors",
            Opacity => "opacity",
            Shadow => "shadow",
            Transform => "transform",
            Custom(s) => s,
        })
    }
}
