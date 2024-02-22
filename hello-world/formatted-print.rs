fn main() {
    // {} replaced with any arguments
    println!("{} days", 31);

    // Positional Arguments!
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "John");

    // Arguments can be named!!
    println!("{subject} {verb} {object}",
    object = "the cat",
    subject = "the quick turtle",
    verb = "jumps over");

    // Different formatting can be invoked by specifying the format character
    println!("Base 10: {}", 60420);
    println!("Base 2: {:b}", 60420);
    println!("Base 8: {:o}", 60420);
    println!("Base 16: {:x}", 60420);
    println!("Base 16: {:X}", 60420);


    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);
    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {pi:.*}", 3)

}