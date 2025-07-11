use path2enum::magic;

#[magic(path = "public", ext = "svg,jpg,png,webp", prefix = "public")]
pub enum Dir {}
