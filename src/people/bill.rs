use crate::person::*;
pub struct Bill;
impl Person for Bill {
    fn greet(&self) {
        println!("Bill")
    }
}
