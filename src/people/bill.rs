use crate::Utils::*;
use ctor::ctor;
use libc_print::*;

pub struct Bill;
impl Person for Bill {
    fn greet(&self) {
        println!("Bill")
    }
}

#[ctor]
fn initialise() {
    PEOPLE.push(&(Bill) as &dyn Person);
    libc_eprintln!("Test");
}
