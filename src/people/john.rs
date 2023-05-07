use crate::person::*;
pub struct John;
impl Person for John {
    fn greet(&self) {
        println!("John")
    }
}
