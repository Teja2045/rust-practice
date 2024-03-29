### Traits

```
#![deny(clippy::all)]

use core::fmt;

struct Person {
    name: String,
    age: u32,
}

trait InitWithName {
    fn new(str: &str) -> Self;
}

impl InitWithName for Person {
    fn new(str: &str) -> Self {
        Person {
            name: str.to_string(),
            age: 23,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn print(value: &impl fmt::Display) {
    print!("printing {}", value);
}
fn main() {
    let person = Person::new("saiteja");

    println!("name is {}, age is {}", person.name, person.age);
    println!("person = {}", person);
    print(&person);
}

```