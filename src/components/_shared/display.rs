use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Display {
    #[default]
    Inherit,
    Block,
    InlineBlock,
    Inline,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    Hidden,
}

impl ToStr for Display {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Display::Inherit => Cow::Borrowed(""),
            Display::Block => Cow::Borrowed("block"),
            Display::InlineBlock => Cow::Borrowed("inline-block"),
            Display::Inline => Cow::Borrowed("inline"),
            Display::Flex => Cow::Borrowed("flex"),
            Display::InlineFlex => Cow::Borrowed("inline-flex"),
            Display::Grid => Cow::Borrowed("grid"),
            Display::InlineGrid => Cow::Borrowed("inline-grid"),
            Display::Hidden => Cow::Borrowed("hidden"),
        }
    }
}
