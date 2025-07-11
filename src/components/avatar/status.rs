#[derive(Clone, Copy)]
pub enum Status {
    Online,
    Offline,
    Busy,
    Absent,
}

impl Status {
    pub fn to_str(&self) -> &'static str {
        match self {
            Status::Online => "online",
            Status::Offline => "offline",
            Status::Busy => "busy",
            Status::Absent => "absent",
        }
    }
}
