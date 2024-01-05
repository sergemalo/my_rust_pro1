#![allow(unused_variables)]
use std::io;

#[doc = r"Crate comment.
My Hello World app"]

fn main() {

#![doc = r#"# MAIN function
This is the doc of my main
```
fn main()
```

Reads user input and spit it out"#]

    println!("Hello, world!");
    println!("{prenom} {Nom}", prenom="Serge", Nom="Malo");
    // Positionnal + Traits:
    println!("Dec: {0}, Hex: 0x{0:x}", 42);
    // Debug Trait
    println!("DBG: {:?}", [1, 2, 3]);

    // Variables
    // Type is optional 
    // snake_case
    let x = "Serge";
    println!("{}", x);
    // Sometimes, type must be specified to avoid errors
    let amount:i64 = 109_876_543_210;
    // immutable by default
    let mut my_val = 42;
    println!("{}", my_val);
    my_val = 21;
    println!("{}", my_val);
    // Shadowing
    let color = "blue";
    let color = 33;
    println!("{}", color);
    // Declare multiple
    let (a, b, c) = (1, 2, 3);
    println!("{}", a);

    // Scalar
    // Char: size is 4 BYTES !
    let smiley = '\u{1F601}';
    println!("{}", smiley);


    // Strings
    // Next are "String slices": they immutable.
    let dog: &str = "Flash";
    let dog2: &'static str = "Flash-static";
    // String object
    let dog3 = String::new();
    let dog3 = String::from(dog);
    // String format Macro
    let mut owner = String::from(format!("Serge is owner of {}", dog3));
    println!("{}", owner);
    // push_str
    // dog.push_str("---"); !!! Impossible on slices - immutable
    owner.push_str(" with love.");

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
