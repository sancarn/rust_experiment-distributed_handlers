use crate::Utils::*;
use ctor::ctor;
use libc_print::*;

pub struct John;
impl Person for John {
    fn greet(&self) {
        println!("John")
    }
}

#[ctor]
fn initialise() {
    PEOPLE.push(&(John) as &dyn Person);
    libc_eprintln!("Test");
}
