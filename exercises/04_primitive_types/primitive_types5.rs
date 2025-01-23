fn main() {
    let mut cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // DONE: Deconstructed the tuple into two variables
    // NOTES: The variables must be declared with the same type as the tuple elements.

    let (name, age) = cat;

    println!("{name} is {age} years old");
    println!("The original tuple is: {:?}", cat);

    // Change the values in the tuple
    // Needed to change the cat tuple to a mutable variable
    cat.0 = "Whiskers";
    cat.1 = 4.0;

    println!("After changing the tuple:");
    println!("{name} is {age} years old");
    println!("The original tuple is: {:?}", cat);

    // The variables 'name' and 'age' are not changed after the tuple is changed
}
