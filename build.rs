use std::{env, fs};
macro_rules! print {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

const TEMPLATE: &str = r#"
// Generated bindings to modules
$[mods]
use crate::person::*;
$[uses]

pub fn get_people() -> Vec<Box<dyn Person>> {
    $[inits]
}
"#;

fn main() {
    let cur = env::current_dir().unwrap();
    let path = String::from(cur.to_string_lossy());
    let mut mods = String::from("");
    let mut uses = String::from("");
    let mut inits: Vec<String> = vec![];
    for entry in fs::read_dir(path.clone() + "/src/people").unwrap() {
        let entry: fs::DirEntry = entry.unwrap();
        let name: String = entry.file_name().into_string().unwrap();
        let name: String = String::from(name.split_at(name.len() - 3).0);
        let mut proper: String = name.clone();
        let proper: String = format!("{}{proper}", proper.remove(0).to_uppercase());
        mods.push_str(&String::from(format!("mod {};", name)));
        uses.push_str(&String::from(format!("use {}::{};", name, proper)));
        inits.push(format!("Box::new({})", proper));
    }
    let inits = format!("return vec![{}];", inits.join(","));
    let mut template = String::from(TEMPLATE);
    template = template.replace("$[mods]", &mods);
    template = template.replace("$[uses]", &uses);
    template = template.replace("$[inits]", &inits);
    let _ = fs::write(path.clone() + "/src/people.rs", template.clone());
    for s in template.split("\n") {
        print!("{s}");
    }
    print!("Written to {path}/src/people.rs")
}
