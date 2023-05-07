mod people;
mod person;
use people::get_people;

fn main() {
    let people = get_people();
    println!("Length {}", people.len());
    for person in people {
        person.greet();
    }
}
