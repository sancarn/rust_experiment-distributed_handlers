mod people;

pub mod Utils {
    pub trait Person {
        fn greet(&self);
    }

    pub const PEOPLE: Vec<&'static dyn Person> = vec![];
}

fn main() {
    println!("Length {}", Utils::PEOPLE.len());
    for person in Utils::PEOPLE {
        println!("Greet");
        person.greet();
    }
}
