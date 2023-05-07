use crate::person::*;
pub struct Stacy;
impl Person for Stacy {
    fn greet(&self) {
        println!("Stacy")
    }
}
