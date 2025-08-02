use crate::prelude::*;

pub struct FormInputContainer;
pub struct FormInputLabel;
pub struct FormInput;
pub struct FormInputMessage;

impl FormInputContainer {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::Flex)
            .padding(TwUnit::Two)
    }
}

impl FormInputLabel {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::Flex)
            .text_size(TwUnit::Sm)
            .font_weight(FontWeight::Medium)
    }
}

impl FormInput {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .rounded(TwUnit::Full)
            .border_width(TwUnit::Two)
            .border_color(Color::White)
    }
}
