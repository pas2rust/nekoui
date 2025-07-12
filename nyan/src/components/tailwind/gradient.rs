use super::{color::Color, dto::ToStr};
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum GradientDirection {
    #[default]
    Inherit,
    ToT,
    ToTr,
    ToR,
    ToBr,
    ToB,
    ToBl,
    ToL,
    ToTl,
}

impl ToStr for GradientDirection {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            GradientDirection::Inherit => Cow::Borrowed(""),
            GradientDirection::ToT => Cow::Borrowed("to-t"),
            GradientDirection::ToTr => Cow::Borrowed("to-tr"),
            GradientDirection::ToR => Cow::Borrowed("to-r"),
            GradientDirection::ToBr => Cow::Borrowed("to-br"),
            GradientDirection::ToB => Cow::Borrowed("to-b"),
            GradientDirection::ToBl => Cow::Borrowed("to-bl"),
            GradientDirection::ToL => Cow::Borrowed("to-l"),
            GradientDirection::ToTl => Cow::Borrowed("to-tl"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum GradientColors {
    #[default]
    Inherit,
    Select {
        from: Color,
        via: Color,
        to: Color,
    },
}

impl GradientColors {
    pub fn new(from: Color, via: Color, to: Color) -> Self {
        Self::Select { from, via, to }
    }
}

impl ToStr for GradientColors {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            GradientColors::Inherit => Cow::Borrowed(""),
            GradientColors::Select { from, via, to } => Cow::Owned(format!(
                "from-{} via-{} to-{}",
                from.to_str(),
                via.to_str(),
                to.to_str()
            )),
        }
    }
}
