
// Generated bindings to modules
mod bill;mod john;mod sarah;mod stacy;
use crate::person::*;
use bill::Bill;use john::John;use sarah::Sarah;use stacy::Stacy;

pub fn get_people() -> Vec<Box<dyn Person>> {
    return vec![Box::new(Bill),Box::new(John),Box::new(Sarah),Box::new(Stacy)];
}
