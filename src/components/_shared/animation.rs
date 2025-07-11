use crate::components::_shared::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Animation {
    #[default]
    Inherit,
    Show,
    RotateLoop,
    ElasticLoop,
    SnowFallLoop,
    SnowFallSlowLoop,
    SnowFallFastLoop,
    RocketLaunchLoop,
    RocketLaunchSlowLoop,
    RocketLaunchFastLoop,
}

impl ToStr for Animation {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Animation::Inherit => Cow::Borrowed(""),
            Animation::Show => Cow::Borrowed("show"),
            Animation::RotateLoop => Cow::Borrowed("rotate-loop"),
            Animation::ElasticLoop => Cow::Borrowed("elastic-loop"),
            Animation::SnowFallLoop => Cow::Borrowed("snow-fall-loop"),
            Animation::SnowFallSlowLoop => Cow::Borrowed("snow-fall-slow-loop"),
            Animation::SnowFallFastLoop => Cow::Borrowed("snow-fall-fast-loop"),
            Animation::RocketLaunchLoop => Cow::Borrowed("rocket-launch-loop"),
            Animation::RocketLaunchSlowLoop => Cow::Borrowed("rocket-launch-slow-loop"),
            Animation::RocketLaunchFastLoop => Cow::Borrowed("rocket-launch-fast-loop"),
        }
    }
}
