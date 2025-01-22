// TODO: Fix the compiler error.
// DONE: Added additional let x: i32; line
// I think this is variable shadowing, but I'm not sure.
fn main() {
    let x = 3;
    println!("Number {x}");

    let x: i32;
    x = 5; // Don't change this line
    println!("Number {x}");
}
