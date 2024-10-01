trait Describale {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Describale for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    println!("{}", person.describe());
}
