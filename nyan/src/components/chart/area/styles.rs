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
    }
}

impl ChartCanvasStyle {
    pub fn normal() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Full)
            .height(TwUnit::Px(500.0))
    }

    pub fn tall() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Full)
            .height(TwUnit::Px(800.0))
    }
}
