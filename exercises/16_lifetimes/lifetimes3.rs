// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
// ADDED: Added a lifetime 'a to the Book struct
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
