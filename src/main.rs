use std::io;

#[doc = r"Crate comment.
My Hello World app"]

fn main() {

#![doc = "# MAIN function\nThis is the doc of my main\n```\nfn main()\n```\n\nReads user input and spit it our"]

    println!("Hello, world!");
    println!("{prenom} {Nom}", prenom="Serge", Nom="Malo");
    // Traits:
    println!("Dec: {0}, Hex: 0x{0:x}", 42);
    // Debug Trait
    println!("DBG: {:?}", [1, 2, 3]);

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
