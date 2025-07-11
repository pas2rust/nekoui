use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Cursor {
    #[default]
    Inherit,
    Auto,
    Default,
    Pointer,
    Wait,
    Text,
    Move,
    Help,
    NotAllowed,
    Crosshair,
    Grab,
    Grabbing,
}

impl ToStr for Cursor {
    fn to_str(&self) -> Cow<'static, str> {
        let s = match self {
            Cursor::Inherit => "",
            Cursor::Auto => "auto",
            Cursor::Default => "default",
            Cursor::Pointer => "pointer",
            Cursor::Wait => "wait",
            Cursor::Text => "text",
            Cursor::Move => "move",
            Cursor::Help => "help",
            Cursor::NotAllowed => "not-allowed",
            Cursor::Crosshair => "crosshair",
            Cursor::Grab => "grab",
            Cursor::Grabbing => "grabbing",
        };
        Cow::Borrowed(s)
    }
}
