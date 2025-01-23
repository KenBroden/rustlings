fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    // DONE: Added unsigned integer 1 as an argument to the call_me function.
    call_me(1);
}
