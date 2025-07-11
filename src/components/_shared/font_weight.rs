use std::borrow::Cow;

use crate::components::_shared::dto::ToStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    #[default]
    Inherit,
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Normal,     // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

impl ToStr for FontWeight {
    fn to_str(&self) -> Cow<'static, str> {
        let s = match self {
            FontWeight::Inherit => "",
            FontWeight::Thin => "thin",
            FontWeight::ExtraLight => "extralight",
            FontWeight::Light => "light",
            FontWeight::Normal => "normal",
            FontWeight::Medium => "medium",
            FontWeight::SemiBold => "semibold",
            FontWeight::Bold => "bold",
            FontWeight::ExtraBold => "extrabold",
            FontWeight::Black => "black",
        };
        Cow::Borrowed(s)
    }
}
