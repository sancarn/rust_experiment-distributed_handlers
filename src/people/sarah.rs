use crate::Utils::*;
use ctor::ctor;
use libc_print::*;

pub struct Sarah;
impl Person for Sarah {
    fn greet(&self) {
        println!("Sarah")
    }
}

#[ctor]
fn initialise() {
    PEOPLE.push(&(Sarah) as &dyn Person);
    libc_eprintln!("Test");
}
