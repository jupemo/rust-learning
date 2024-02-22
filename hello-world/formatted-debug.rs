// This structure that cannot be printed
// struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation required to make this `struct` printable
// #[derive(Debug)]
// struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside the structure `Deep`. Make it printable
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing with {:?} it similar to {}
    println!("{:?} moths in a year", 12);
    println!("{} moths in a year", 12);
    println!("{:?} moths in a year", "12");
    println!("{} moths in a year", "12");

    println!("{1:?}, {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // Structure is printable
    println!("Now {:?} will print", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter  = Person {name, age};

    println!("{:?}", peter);
    println!("{:#?}", peter);
}