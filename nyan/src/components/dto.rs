use crate::prelude::Class;

pub trait BuildClass {
    fn build() -> Vec<Class>;
}
