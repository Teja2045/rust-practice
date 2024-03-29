## Errors

```
#![deny(clippy::all)]

fn first_name() -> Result<String, String> {
    Ok("saiteja".to_string())
}

fn last_name() -> Result<String, String> {
    Ok("ettedi".to_string())
}

fn full_name() -> Result<String, String> {
    let first_name = first_name()?;
    let last_name = last_name()?;

    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let name = &full_name();

    let full_name = name.as_ref().expect_err("why name is here?");

    println!("name is {}", full_name);

    match name {
        Ok(name) => println!("name is {}", name),
        Err(_) => println!("unknown error"),
    }
}
```