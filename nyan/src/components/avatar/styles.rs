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
            .scale(TwUnit::OneHundredRaw)
            .transition(Transition::All)
            .duration(TwUnit::ThreeHundred)
            .hover(
                TailwindStyles::new()
                    .opacity(TwUnit::Sixty)
                    .transition(Transition::All)
                    .scale(TwUnit::OneHundredTenRaw)
                    .duration(TwUnit::ThreeHundred)
                    .to_box(),
            )
            .rounded(TwUnit::Full)
            .shadow_size(TwUnit::Lg)
            .shadow_color(Color::Black)
    }
}

impl AvatarStyle {
    pub fn base() -> TailwindStyles {
        TailwindStyles::new()
            .object_fit(ObjectFit::Cover)
            .rounded(TwUnit::Full)
            .padding(TwUnit::Half)
            .shadow_size(TwUnit::Md)
            .ring_size(TwUnit::Two)
            .width(TwUnit::Twelve)
            .height(TwUnit::Twelve)
    }

    pub fn xs() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twelve)
            .height(TwUnit::Twelve)
    }

    pub fn sm() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Sixteen)
            .height(TwUnit::Sixteen)
    }

    pub fn md() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twenty)
            .height(TwUnit::Twenty)
    }

    pub fn lg() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::TwentyFour)
            .height(TwUnit::TwentyFour)
    }

    pub fn xl() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::TwentyEight)
            .height(TwUnit::TwentyEight)
    }

    pub fn xxl() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::ThirtyTwo)
            .height(TwUnit::ThirtyTwo)
    }

    pub fn online() -> Class {
        Class::new()
            .light(Self::base().ring_color(Color::Green(Shade::FiveHundred)))
            .dark(Self::base().ring_color(Color::Green(Shade::FiveHundred)))
            .monokai(Self::base().ring_color(Color::Fuchsia(Shade::FiveHundred)))
            .tokyo(Self::base().ring_color(Color::Purple(Shade::FiveHundred)))
            .dracula(Self::base().ring_color(Color::Pink(Shade::FiveHundred)))
            .carbon(Self::base().ring_color(Color::Green(Shade::FiveHundred)))
            .rust(Self::base().ring_color(Color::Orange(Shade::FiveHundred)))
            .neko(Self::base().ring_color(Color::Violet(Shade::FiveHundred)))
    }

    pub fn busy() -> Class {
        Class::new()
            .light(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .dark(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .monokai(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .tokyo(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .dracula(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .carbon(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .rust(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
            .neko(Self::base().ring_color(Color::Red(Shade::FiveHundred)))
    }

    pub fn absent() -> Class {
        Class::new()
            .light(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .dark(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .monokai(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .tokyo(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .dracula(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .carbon(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .rust(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
            .neko(Self::base().ring_color(Color::Yellow(Shade::FiveHundred)))
    }

    pub fn offline() -> Class {
        Class::new()
            .light(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .dark(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .monokai(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .tokyo(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .dracula(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .carbon(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .rust(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
            .neko(Self::base().ring_color(Color::Gray(Shade::FiveHundred)))
    }
}

impl AvatarDotStyle {
    pub fn base() -> TailwindStyles {
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
            .left(TwUnit::Ten)
    }

    pub fn offline() -> Class {
        Class::new()
            .light(Self::base().bg_color(Color::Gray(Shade::SixHundred)))
            .dark(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
            .monokai(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
            .tokyo(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
            .dracula(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
            .carbon(Self::base().bg_color(Color::Gray(Shade::SevenHundred)))
            .rust(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
            .neko(Self::base().bg_color(Color::Gray(Shade::FiveHundred)))
    }

    pub fn busy() -> Class {
        Class::new()
            .light(Self::base().bg_color(Color::Red(Shade::SixHundred)))
            .dark(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
            .monokai(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
            .tokyo(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
            .dracula(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
            .carbon(Self::base().bg_color(Color::Red(Shade::SevenHundred)))
            .rust(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
            .neko(Self::base().bg_color(Color::Red(Shade::FiveHundred)))
    }

    pub fn absent() -> Class {
        Class::new()
            .light(Self::base().bg_color(Color::Yellow(Shade::SixHundred)))
            .dark(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
            .monokai(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
            .tokyo(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
            .dracula(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
            .carbon(Self::base().bg_color(Color::Yellow(Shade::SevenHundred)))
            .rust(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
            .neko(Self::base().bg_color(Color::Yellow(Shade::FiveHundred)))
    }

    pub fn online() -> Class {
        Class::new()
            .light(Self::base().bg_color(Color::Green(Shade::FiveHundred)))
            .dark(Self::base().bg_color(Color::Green(Shade::FiveHundred)))
            .monokai(Self::base().bg_color(Color::Fuchsia(Shade::FiveHundred)))
            .tokyo(Self::base().bg_color(Color::Purple(Shade::FiveHundred)))
            .dracula(Self::base().bg_color(Color::Pink(Shade::FiveHundred)))
            .carbon(Self::base().bg_color(Color::Green(Shade::SevenHundred)))
            .rust(Self::base().bg_color(Color::Orange(Shade::FiveHundred)))
            .neko(Self::base().bg_color(Color::Violet(Shade::FiveHundred)))
    }

    pub fn xs() -> TailwindStyles {
        Self::base().left(TwUnit::Ten)
    }

    pub fn sm() -> TailwindStyles {
        Self::base().left(TwUnit::Px(54.0))
    }

    pub fn md() -> TailwindStyles {
        Self::base().left(TwUnit::Sixteen)
    }

    pub fn lg() -> TailwindStyles {
        Self::base().left(TwUnit::Px(75.0))
    }

    pub fn xl() -> TailwindStyles {
        Self::base().left(TwUnit::Px(90.0))
    }

    pub fn xxl() -> TailwindStyles {
        Self::base().left(TwUnit::Px(105.0))
    }
}

impl BuildClass for AvatarStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![
            AvatarStyle::online().to_box(),
            AvatarStyle::offline().to_box(),
            AvatarStyle::busy().to_box(),
            AvatarStyle::absent().to_box(),
            AvatarDotStyle::offline().to_box(),
            AvatarDotStyle::online().to_box(),
            AvatarDotStyle::busy().to_box(),
            AvatarDotStyle::absent().to_box(),
            Self::xs().to_box(),
            Self::sm().to_box(),
            Self::md().to_box(),
            Self::lg().to_box(),
            Self::xl().to_box(),
            Self::xxl().to_box(),
            AvatarDotStyle::xs().to_box(),
            AvatarDotStyle::sm().to_box(),
            AvatarDotStyle::md().to_box(),
            AvatarDotStyle::lg().to_box(),
            AvatarDotStyle::xl().to_box(),
            AvatarDotStyle::xxl().to_box(),
            AvatarContainerStyle::new().to_box(),
        ]
    }
}
