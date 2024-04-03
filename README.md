# rust-practice


#### Create Rust Folder

        cargo new [project_name]

#### Installations

- clippy (like a linter)

        rustup component add clippy

        // to add clippy
        #[!deny(clippy::all)]

- cargo watch (to run files)

        cargo install cargo-watch


#### Run

- using cargo-watch

    cargo-watch -qc -x run -x clippy


#### Variables
- let for immutable variables
- let mut for immutable variables

- shadowing: redeclaring variables (data types could be different)

- constants

- tuples

```
#![deny(clippy::all)]

fn main() {
    let name = "Foo";

    println!("Your name is  {}", name);

    let personal_data = (22, "Teja");
    let (_age, name) = personal_data;
    let _age = personal_data.0;

    println!("your data: {}", name)
}

```

#### Ownership

keywords

- borrowing, moving, dangling reference, mutable reference vs immutable reference

Moving
```
#![deny(clippy::all)]

fn empty_string(str: String) {
    println!("your string is {}", str);
    // str.clear()
}
fn main() {
    let name = String::from("Sai");
    let mut name1 = String::from("teja");
    let name2 = &name;

    empty_string(name);

    // moved so can't barrow like this
    println!("name is {}", name);
}
```

Mutability
```
fn empty_string(str: &mut String) {
    str.clear()
    // str.clear()
}
fn main() {
    let mut name = String::from("Sai");

    println!("name is {}", name);
    empty_string(&mut name);
    println!("name is {}", name);
}

```


#### structs example 

```
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Teja".to_string(),
        age: 21,
    };

    let person2 = Person {
        name: "Sai".to_string(),
        ..person
    };

    println!(
        "person name is {} and age is {} and person2 name is {}",
        person.name, person.age, person2.name
    );
}
```

```
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn print(&self) {
        println!("name is {}", self.name);
    }

    fn person_with_double_age(&self) -> Person {
        Person {
            name: "randome".to_string(),
            age: self.age * 2,
        }
    }
}

fn main() {
    let person = Person {
        name: "Teja".to_string(),
        age: 21,
    };

    let person2 = Person {
        name: "Sai".to_string(),
        ..person
    };

    println!(
        "person name is {} and age is {} and person2 name is {}",
        person.name, person.age, person2.name
    );

    person.print();

    let person3 = person.person_with_double_age();

    println!("doubled person is {}", person3.name)
}
```

keywords: tuples = struct Point(f64, f64, f64)
