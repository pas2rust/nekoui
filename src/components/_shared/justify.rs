use crate::components::_shared::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    #[default]
    Inherit,
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

impl ToStr for JustifyContent {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            JustifyContent::Inherit => "",
            JustifyContent::Start => "start",
            JustifyContent::End => "end",
            JustifyContent::Center => "center",
            JustifyContent::Between => "between",
            JustifyContent::Around => "around",
            JustifyContent::Evenly => "evenly",
        };
        Cow::Borrowed(st)
    }
}
