use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    #[default]
    Inherit,
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

impl ToStr for TextAlign {
    fn to_str(&self) -> Cow<'static, str> {
        let s = match self {
            TextAlign::Inherit => "",
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "justify",
            TextAlign::Start => "start",
            TextAlign::End => "end",
        };
        Cow::Borrowed(s)
    }
}
