use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Tracking {
    #[default]
    Inherit,
    Tighter,
    Tight,
    Wide,
    Wider,
    Widest,
}

impl ToStr for Tracking {
    fn to_str(&self) -> Cow<'static, str> {
        let s = match self {
            Tracking::Inherit => "",
            Tracking::Tighter => "tighter",
            Tracking::Tight => "tight",
            Tracking::Wide => "wide",
            Tracking::Wider => "wider",
            Tracking::Widest => "widest",
        };
        Cow::Borrowed(s)
    }
}
