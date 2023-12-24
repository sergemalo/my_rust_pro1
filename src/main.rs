use std::io;

#[doc = r"Crate comment.
My Hello World app"]

fn main() {

#![doc = r"# MAIN function
This is the doc of my main
```
fn main()
```

Reads user input and spit it our"]

    println!("Hello, world!");

    let mut input = String::new();
    println!("Say something:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }
}
