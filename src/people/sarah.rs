use crate::person::*;
pub struct Sarah;
impl Person for Sarah {
    fn greet(&self) {
        println!("Sarah")
    }
}
