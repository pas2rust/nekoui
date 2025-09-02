use crate::prelude::{InputValid, use_ctx};

pub fn use_input_valid_ctx() -> InputValid {
    use_ctx::<InputValid>().expect("Form input valid must be provided!")
}
