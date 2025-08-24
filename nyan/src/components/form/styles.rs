use crate::prelude::*;

pub struct FormInputContainerStyle;
pub struct FormInputLabelStyle;
pub struct FormInputTextStyle;
pub struct FormInputMessageStyle;
pub struct FormInputCardStyle;

impl FormInputCardStyle {
    pub fn class() -> Class {
        Class::new()
            .light(Self::new())
    }
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::InlineFlex)
            .align_items(AlignItems::Center)
            .px(TwUnit::Three)
    }
}


impl FormInputContainerStyle {
    pub fn class() -> Class {
        Class::new()
            .light(Self::new())
    }
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::Flex)
            .padding(TwUnit::Two)
    }
}

impl FormInputLabelStyle {
    pub fn new() -> TailwindStyles {
         TailwindStyles::new()
            .display(Display::Flex)
            .text_size(TwUnit::Sm)
            .font_weight(FontWeight::Medium)
            .text_color(Color::White)
    }
    pub fn class() -> Class {
          Class::new()
            .light(Self::new())
    }
    pub fn class_success() -> Class {
         Class::new()
            .light(Self::new().text_color(Color::Green(Shade::FiveHundred)))
    }
    pub fn class_error() -> Class {
         Class::new()
            .light(Self::new().text_color(Color::Red(Shade::FiveHundred)))
    }
}

impl FormInputTextStyle {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .rounded(TwUnit::Full)
            .border_width(TwUnit::Two)
            .border_color(Color::White)
    }
}

impl BuildClass for FormInputLabelStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![
            FormInputLabelStyle::new().to_box(),
            FormInputLabelStyle::class().to_box(),
            FormInputLabelStyle::class_success().to_box(),
            FormInputLabelStyle::class_error().to_box()
        ]
    }
}