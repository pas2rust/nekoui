#[derive(Debug, Default, Clone)]
pub enum ToastKind {
    #[default]
    None,
    Info {
        message: String,
        title: String,
    },
    Error {
        message: String,
        title: String,
    },
    Success {
        message: String,
        title: String,
    },
    Warning {
        message: String,
        title: String,
    },
}
