fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        //let z = &mut x;
        y.push(42);
        assert_eq!(y, &vec![42]);

        let z = &mut x; // This line moved here to compile.
        z.push(13);
        assert_eq!(x, [42, 13]);
        // assert_eq!(y, &vec![42]); // This line won't compile because y is borrowed.
    }
}
