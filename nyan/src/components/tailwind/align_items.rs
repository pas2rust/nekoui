use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    #[default]
    Inherit,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl ToStr for AlignItems {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            AlignItems::Inherit => "",
            AlignItems::Start => "start",
            AlignItems::End => "end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Stretch => "stretch",
        };
        Cow::Borrowed(st)
    }
}
