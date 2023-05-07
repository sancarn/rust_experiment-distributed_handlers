// Manual binding to structs here

mod bill;
mod john;
mod sarah;
use crate::person::*;
use bill::Bill;
use john::John;
use sarah::Sarah;

pub fn get_people() -> Vec<Box<dyn Person>> {
    return vec![Box::new(Bill), Box::new(John), Box::new(Sarah)];
}
