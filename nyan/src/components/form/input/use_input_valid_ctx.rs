use crate::prelude::{use_ctx, InputValid};

pub fn use_input_valid_ctx() -> InputValid {
    use_ctx::<InputValid>().expect("Form input valid must be provided!")
}