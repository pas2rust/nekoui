use super::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TwUnit {
    #[default]
    Inherit,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    TwoXl,
    ThreeXl,
    FourXl,
    FiveXl,
    SixXl,
    SevenXl,
    EightXl,
    NineXl,
    Zero,                    // 0
    ZeroPointFive,           // 0.5
    One,                     // 1
    Two,                     // 2
    TwoHalf,                 // 2.5
    Three,                   // 3
    Four,                    // 4
    Five,                    // 5
    Six,                     // 6
    Seven,                   // 7
    Eight,                   // 8
    Nine,                    // 9
    Ten,                     // 10
    Eleven,                  // 11
    Twelve,                  // 12
    Fourteen,                // 14
    Sixteen,                 // 16
    Twenty,                  // 20
    TwentyFour,              // 24
    TwentyEight,             // 28
    ThirtyTwo,               // 32
    ThirtySix,               // 36
    Forty,                   // 40
    FortyFour,               // 44
    FortyEight,              // 48
    FiftyTwo,                // 52
    FiftySix,                // 56
    Sixty,                   // 60
    SixtyFour,               // 64
    SeventyTwo,              // 72
    Eighty,                  // 80
    NinetySix,               // 96
    Half,                    // 1/2
    OneThird,                // 1/3
    TwoThirds,               // 2/3
    OneFourth,               // 1/4
    ThreeFourths,            // 3/4
    OneFifth,                // 1/5
    TwoFifths,               // 2/5
    ThreeFifths,             // 3/5
    FourFifths,              // 4/5
    OneSixth,                // 1/6
    FiveSixths,              // 5/6,
    Fifty,                   // 50%
    FiftyRaw,                // 50
    SeventyFive,             // 75%
    SeventyFiveRaw,          // 75
    Ninety,                  // 90%
    NinetyRaw,               // 90
    NinetyFive,              // 95%
    NinetyFiveRaw,           // 95
    OneHundred,              // 100%
    OneHundredRaw,           // 100
    OneHundredFive,          // 105%
    OneHundredFiveRaw,       // 105
    OneHundredTen,           // 110%
    OneHundredTenRaw,        // 110
    OneHundredTwentyFive,    // 125%
    OneHundredTwentyFiveRaw, // 125
    OneHundredFifty,         // 150%
    OneHundredFiftyRaw,      // 150
    ThreeHundred,            // 300%
    ThreeHundredRaw,         // 300
    Full,
    Fit,
    Px(f32),
    Percent(f32),
    Em(f32),
    Rem(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
    Ch(f32),
    Ex(f32),
    Deg(f32),
    Ms(f32),
    S(f32),
    Custom(&'static str),
}

impl ToStr for TwUnit {
    fn to_str(&self) -> Cow<'static, str> {
        use TwUnit::*;
        match self {
            Inherit => Cow::Borrowed(""),
            Xs => Cow::Borrowed("xs"),
            Sm => Cow::Borrowed("sm"),
            Md => Cow::Borrowed("md"),
            Lg => Cow::Borrowed("lg"),
            Xl => Cow::Borrowed("xl"),
            TwoXl => Cow::Borrowed("2xl"),
            ThreeXl => Cow::Borrowed("3xl"),
            FourXl => Cow::Borrowed("4xl"),
            FiveXl => Cow::Borrowed("5xl"),
            SixXl => Cow::Borrowed("6xl"),
            SevenXl => Cow::Borrowed("7xl"),
            EightXl => Cow::Borrowed("8xl"),
            NineXl => Cow::Borrowed("9xl"),
            Zero => Cow::Borrowed("0"),
            ZeroPointFive => Cow::Borrowed("0.5"),
            One => Cow::Borrowed("1"),
            Two => Cow::Borrowed("2"),
            TwoHalf => Cow::Borrowed("2.5"),
            Three => Cow::Borrowed("3"),
            Four => Cow::Borrowed("4"),
            Five => Cow::Borrowed("5"),
            Six => Cow::Borrowed("6"),
            Seven => Cow::Borrowed("7"),
            Eight => Cow::Borrowed("8"),
            Nine => Cow::Borrowed("9"),
            Ten => Cow::Borrowed("10"),
            Eleven => Cow::Borrowed("11"),
            Twelve => Cow::Borrowed("12"),
            Fourteen => Cow::Borrowed("14"),
            Sixteen => Cow::Borrowed("16"),
            Twenty => Cow::Borrowed("20"),
            TwentyFour => Cow::Borrowed("24"),
            TwentyEight => Cow::Borrowed("28"),
            ThirtyTwo => Cow::Borrowed("32"),
            ThirtySix => Cow::Borrowed("36"),
            Forty => Cow::Borrowed("40"),
            FortyFour => Cow::Borrowed("44"),
            FortyEight => Cow::Borrowed("48"),
            FiftyTwo => Cow::Borrowed("52"),
            FiftySix => Cow::Borrowed("56"),
            Sixty => Cow::Borrowed("60"),
            SixtyFour => Cow::Borrowed("64"),
            SeventyTwo => Cow::Borrowed("72"),
            Eighty => Cow::Borrowed("80"),
            NinetySix => Cow::Borrowed("96"),
            Half => Cow::Borrowed("1/2"),
            OneThird => Cow::Borrowed("1/3"),
            TwoThirds => Cow::Borrowed("2/3"),
            OneFourth => Cow::Borrowed("1/4"),
            ThreeFourths => Cow::Borrowed("3/4"),
            OneFifth => Cow::Borrowed("1/5"),
            TwoFifths => Cow::Borrowed("2/5"),
            ThreeFifths => Cow::Borrowed("3/5"),
            FourFifths => Cow::Borrowed("4/5"),
            OneSixth => Cow::Borrowed("1/6"),
            FiveSixths => Cow::Borrowed("5/6"),
            Full => Cow::Borrowed("full"),
            Fit => Cow::Borrowed("fit"),
            Fifty => Cow::Borrowed("50%"),
            FiftyRaw => Cow::Borrowed("50"),
            SeventyFive => Cow::Borrowed("75%"),
            SeventyFiveRaw => Cow::Borrowed("75"),
            Ninety => Cow::Borrowed("90%"),
            NinetyRaw => Cow::Borrowed("90"),
            NinetyFive => Cow::Borrowed("95%"),
            NinetyFiveRaw => Cow::Borrowed("95"),
            OneHundred => Cow::Borrowed("100%"),
            OneHundredRaw => Cow::Borrowed("100"),
            OneHundredFive => Cow::Borrowed("105%"),
            OneHundredFiveRaw => Cow::Borrowed("105"),
            OneHundredTen => Cow::Borrowed("110%"),
            OneHundredTenRaw => Cow::Borrowed("110"),
            OneHundredTwentyFive => Cow::Borrowed("125%"),
            OneHundredTwentyFiveRaw => Cow::Borrowed("125"),
            OneHundredFifty => Cow::Borrowed("150%"),
            OneHundredFiftyRaw => Cow::Borrowed("150"),
            ThreeHundred => Cow::Borrowed("300%"),
            ThreeHundredRaw => Cow::Borrowed("300"),
            Px(val) => Cow::Owned(format!("[{}px]", val)),
            Percent(val) => Cow::Owned(format!("[{}%]", val)),
            Em(val) => Cow::Owned(format!("[{}em]", val)),
            Rem(val) => Cow::Owned(format!("[{}rem]", val)),
            Vw(val) => Cow::Owned(format!("[{}vw]", val)),
            Vh(val) => Cow::Owned(format!("[{}vh]", val)),
            Vmin(val) => Cow::Owned(format!("[{}vmin]", val)),
            Vmax(val) => Cow::Owned(format!("[{}vmax]", val)),
            Ch(val) => Cow::Owned(format!("[{}ch]", val)),
            Ex(val) => Cow::Owned(format!("[{}ex]", val)),
            Deg(val) => Cow::Owned(format!("[{}deg]", val)),
            Ms(val) => Cow::Owned(format!("[{}ms]", val)),
            S(val) => Cow::Owned(format!("[{}s]", val)),
            Custom(val) => Cow::Owned(format!("[{}]", val)),
        }
    }
}
