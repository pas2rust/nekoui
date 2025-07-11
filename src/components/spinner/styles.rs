use crate::components::_shared::prelude::*;

pub struct SpinnerStyle;

impl SpinnerStyle {
    pub fn base() -> TailwindStyles {
        TailwindStyles::new()
            .display(Display::InlineBlock)
            .animation(Animation::Spin)
            .text_color(Color::Gray(Shade::TwoHundred))
            .fill(Color::Blue(Shade::FiveHundred))
    }

    pub fn xs() -> TailwindStyles {
        Self::base().width(TwUnit::Four).height(TwUnit::Four)
    }

    pub fn sm() -> TailwindStyles {
        Self::base().width(TwUnit::Six).height(TwUnit::Six)
    }

    pub fn md() -> TailwindStyles {
        Self::base().width(TwUnit::Eight).height(TwUnit::Eight)
    }

    pub fn lg() -> TailwindStyles {
        Self::base().width(TwUnit::Ten).height(TwUnit::Ten)
    }

    pub fn xl() -> TailwindStyles {
        Self::base().width(TwUnit::Twelve).height(TwUnit::Twelve)
    }

    pub fn x2l() -> TailwindStyles {
        Self::base().width(TwUnit::Sixteen).height(TwUnit::Sixteen)
    }
}
