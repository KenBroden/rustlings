fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // DONE: Defined an array with 100 elements
    // arrays in rust: let variable_name: [type; number_of_elements] = [element1, element2, ...];

    let a: [i32; 100]= [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
