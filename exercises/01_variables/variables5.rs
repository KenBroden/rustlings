fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // DONE: Another case of variable shadowing I think?
    // Not sure what happens to the number string variable,
    let number: i32;
    number = 3;
    println!("Number plus two is: {}", number + 2);
}
