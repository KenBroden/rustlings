fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    // DONE: Substituted 2.0 with 2. Removed else "Unknown" string, replaced with 4, could be any int not equal to 1, 2, or 3.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
    let a = "crab";
    let b = "gopher";
    let c = "snake";
    let d = "dinosaur";

    println!("{}", animal_habitat(a));
    println!("{}", animal_habitat(b));
    println!("{}", animal_habitat(c));
    println!("{}", animal_habitat(d));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
