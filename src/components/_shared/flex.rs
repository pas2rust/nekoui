use crate::components::_shared::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    #[default]
    Inherit,
    Row,
    RowReverse,
    Col,
    ColReverse,
}

impl ToStr for FlexDirection {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            FlexDirection::Inherit => "",
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Col => "col",
            FlexDirection::ColReverse => "col-reverse",
        };
        Cow::Borrowed(st)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    #[default]
    Inherit,
    Wrap,
    WrapReverse,
    NoWrap,
}

impl ToStr for FlexWrap {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            FlexWrap::Inherit => "",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
            FlexWrap::NoWrap => "nowrap",
        };
        Cow::Borrowed(st)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FlexGrow {
    #[default]
    Inherit,
    Grow,
    Grow0,
}

impl ToStr for FlexGrow {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            FlexGrow::Inherit => "",
            FlexGrow::Grow => "",
            FlexGrow::Grow0 => "0",
        };
        Cow::Borrowed(st)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FlexShrink {
    #[default]
    Inherit,
    Shrink,
    Shrink0,
}

impl ToStr for FlexShrink {
    fn to_str(&self) -> Cow<'static, str> {
        let st = match self {
            FlexShrink::Inherit => "",
            FlexShrink::Shrink => "",
            FlexShrink::Shrink0 => "0",
        };
        Cow::Borrowed(st)
    }
}
