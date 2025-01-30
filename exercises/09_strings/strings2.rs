// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    // ADDED: & to word to pass a reference to the string slice.
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    // Demonstrate that the original String is still accessible
    println!("The original word is still accessible: {}", word);

    // Modify the original String to show it is still owned by `main`
    let mut word = word;
    word.push_str(" is a nice color");
    println!("Modified original word: {}", word);
}
