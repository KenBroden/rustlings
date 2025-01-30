// TODO: Fix the compiler error without changing the function signature.
// ADDED: .to_string() method to convert the string slice to a String.
fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
