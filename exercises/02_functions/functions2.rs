// TODO: Add the missing type of the argument `num` after the colon `:`.
// DONE: Added i32 type to the num argument.
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
