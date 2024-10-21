use nanoid::{alphabet, nanoid};

pub struct Id {}

impl Id {
    pub fn gen() -> String {
        nanoid!()
    }
}
