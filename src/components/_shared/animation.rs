use crate::components::_shared::dto::ToStr;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Animation {
    #[default]
    Inherit,
    Spin,
    Ping,
    Pulse,
    Bounce,
}

impl ToStr for Animation {
    fn to_str(&self) -> Cow<'static, str> {
        match self {
            Animation::Inherit => Cow::Borrowed(""),
            Animation::Spin => Cow::Borrowed("spin"),
            Animation::Ping => Cow::Borrowed("ping"),
            Animation::Pulse => Cow::Borrowed("pulse"),
            Animation::Bounce => Cow::Borrowed("bounce"),
        }
    }
}
