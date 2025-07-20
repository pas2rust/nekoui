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
            .width(TwUnit::Px(300.0))
    }

    pub fn card() -> TailwindStyles {
        TailwindStyles::new()
            .border_width(TwUnit::Px(1.0))
            .border_color(Color::Gray(Shade::ThreeHundred))
            .rounded(TwUnit::Lg)
            .shadow_size(TwUnit::Md)
            .width(TwUnit::Px(300.0))
            .margin(TwUnit::Four)
    }
}
