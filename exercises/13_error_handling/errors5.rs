// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context. For this exercise, that context is the potential errors which
// can be returned in a `Result`.

use std::error::Error;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CreationError::Negative => write!(f, "Negative value provided"),
            CreationError::Zero => write!(f, "Zero value provided"),
        }
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement?
// ADDED: return type Result<(), Box<dyn Error>> - allows for multiple error types
fn main() -> Result<(), Box<dyn Error>> {

    // Testing the dynamic error handling
    let test_inputs = vec!["42", "-42", "0", "not a number"];

    for input in test_inputs {
        match run_test(input) {
            Ok(result) => println!("Input: {}, Result: {:?}", input, result),
            Err(e) => println!("Input: {}, Error: {}", input, e),
        }
    }
    Ok(())
}

fn run_test(input: &str) -> Result<PositiveNonzeroInteger, Box<dyn Error>> {
    let x: i64 = input.parse()?;
    let result = PositiveNonzeroInteger::new(x)?;
    Ok(result)
}
