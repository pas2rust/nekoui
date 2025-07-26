use crate::prelude::*;

pub struct FormInputContainer;
pub struct FormInputLabel;

impl FormInputContainer {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new().display(Display::Flex)
    }
}
