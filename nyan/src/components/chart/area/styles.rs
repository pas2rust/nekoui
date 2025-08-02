use crate::components::prelude::*;

pub struct ChartAreaStyle;
pub struct ChartCanvasStyle;

impl ChartAreaStyle {
    pub fn container() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::Flex)
            .justify_content(JustifyContent::Center)
            .flex_wrap(FlexWrap::Wrap)
            .gap(TwUnit::Four)
    }

    pub fn card() -> TailwindStyles {
        TailwindStyles::new()
            .border_width(TwUnit::Px(1.0))
            .border_color(Color::Gray(Shade::ThreeHundred))
            .rounded(TwUnit::Lg)
            .shadow_size(TwUnit::Md)
            .width(TwUnit::Px(400.0))
            .height(TwUnit::Px(400.0))
    }
}

impl BuildClass for ChartAreaStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![Self::container().to_box(), Self::card().to_box()]
    }
}
