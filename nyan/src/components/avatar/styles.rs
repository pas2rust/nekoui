use crate::components::prelude::*;

pub struct AvatarStyle;
pub struct AvatarDotStyle;
pub struct AvatarContainerStyle;

impl AvatarContainerStyle {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .position(Position::Relative)
            .display(Display::Flex)
            .width(TwUnit::Fit)
            .height(TwUnit::Fit)
            .margin(TwUnit::Four)
            .cursor(Cursor::Pointer)
            .scale(TwUnit::OneHundred)
            .transition(Transition::All)
            .duration(TwUnit::ThreeHundred)
            .hover(
                TailwindStyles::new()
                    .opacity(TwUnit::Sixty)
                    .transition(Transition::All)
                    .scale(TwUnit::OneHundredTen)
                    .duration(TwUnit::ThreeHundred)
                    .to_box(),
            )
            .rounded(TwUnit::Full)
            .shadow_size(TwUnit::Lg)
            .shadow_color(Color::Black)
    }
}

impl AvatarStyle {
    pub fn offline() -> TailwindStyles {
        TailwindStyles::new()
            .object_fit(ObjectFit::Cover)
            .rounded(TwUnit::Full)
            .padding(TwUnit::Half)
            .shadow_size(TwUnit::Md)
            .ring_size(TwUnit::Two)
            .ring_color(Color::Gray(Shade::FiveHundred))
    }

    pub fn xs() -> TailwindStyles {
        Self::offline().width(TwUnit::Twelve).height(TwUnit::Twelve)
    }

    pub fn sm() -> TailwindStyles {
        Self::offline()
            .width(TwUnit::Sixteen)
            .height(TwUnit::Sixteen)
    }

    pub fn md() -> TailwindStyles {
        Self::offline().width(TwUnit::Twenty).height(TwUnit::Twenty)
    }

    pub fn lg() -> TailwindStyles {
        Self::offline()
            .width(TwUnit::TwentyFour)
            .height(TwUnit::TwentyFour)
    }

    pub fn xl() -> TailwindStyles {
        Self::offline()
            .width(TwUnit::TwentyEight)
            .height(TwUnit::TwentyEight)
    }

    pub fn xxl() -> TailwindStyles {
        Self::offline()
            .width(TwUnit::ThirtyTwo)
            .height(TwUnit::ThirtyTwo)
    }
}

impl AvatarDotStyle {
    pub fn offline() -> TailwindStyles {
        TailwindStyles::new()
            .position(Position::Absolute)
            .bg_color(Color::Gray(Shade::FiveHundred))
            .rounded(TwUnit::Full)
            .ring_size(TwUnit::One)
            .ring_color(Color::White)
            .padding(TwUnit::Two)
            .width(TwUnit::Percent(20.0))
            .height(TwUnit::Percent(20.0))
            .top(TwUnit::Zero)
    }

    pub fn xs() -> TailwindStyles {
        Self::offline().left(TwUnit::Ten)
    }

    pub fn sm() -> TailwindStyles {
        Self::offline().left(TwUnit::Px(54.0))
    }

    pub fn md() -> TailwindStyles {
        Self::offline().left(TwUnit::Sixteen)
    }

    pub fn lg() -> TailwindStyles {
        Self::offline().left(TwUnit::Px(75.0))
    }

    pub fn xl() -> TailwindStyles {
        Self::offline().left(TwUnit::Px(90.0))
    }

    pub fn xxl() -> TailwindStyles {
        Self::offline().left(TwUnit::Px(105.0))
    }
}

impl BuildClass for AvatarStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![]
    }
}
