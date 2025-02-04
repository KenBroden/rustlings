# Rustling Exercises

*Completed by Ken Broden*

## Overview

This is my attempt at learning the Rust programming language. I am going to be attempting these exercises, learning Rust, and writing notes here in this README in concurrence with my Memory Safe Programming class CSCI 4611.

## Exercise Notes

### Week 1

#### Exercise 00: Introduction

- **Exercise:** [00_intro](exercises/00_intro)
- **Solution:**
  - Fixed print statement.
- **Questions/Issues:**

#### Exercise 01: Variables

- **Exercise:** [01_variables](exercises/01_variables)
- **Issue:** All exercises did not compile.
- **Solution:**
  - variables1: Added let statement
  - variables2: Added a value of 10 to x
  - variables3: Added value of 10 to x
  - variables4: Added additional 'let x: i32'. Example of variable shadowing
  - variables5: Added let number: i32;
  - variables6: Added variable type to NUMBER
- **Explanation:**
  - Most of the solutions are basic initialization techniques in Rust. Variable shadowing showed up twice, although the actual example implementation seemed a little pointless.
- **Questions/Issues/Notes:**
  - variable4: why would you variable shadow with the same type?
  - const variables are always immutable and must have a type.

#### Exercise 02: Functions

- **Exercise:** [02_functions](exercises/02_functions)
- **Issue:** All exercises did not compile.
- **Solution:**
  - function1: Defined function to print string
  - function2: Declared data type of num parameter
  - function3: Added unsigned integer 1 to the function call
  - function4: Added return type to sales_price function.
  - function5: Removed semicolon in function, makes result of that line the return val of function
- **Explanation:**
  - Solutions are mostly applicable to a majority of programming languages. function5 was unique to Rust, last line of code without ; is return statement
- **Questions/Issues/Notes:**
  - Everything compiles.

### Week 2

#### Exercise 03: If

- **Exercise:** [03_if](exercises/03_if)
- **Issue:** Need to write if/else statement for functions to compile
- **Solution:**
  - if1: Wrote if/else for a > b
  - if2: Wrote else statements for the two other output options
  - if3: Substituted 2.0 with 2. Removed else "Unknown" string, replaced with
- **Explanation:**
  - Solutions are based on the if / else if / else format of Rust. Nothing too novel. I think i still prefer explicitly writing
  'return' like I do in Java.
- **Questions/Issues/Notes:**
  - All tests pass. Everything compiles.

#### Exercise 04: Primitive Types

- **Exercise:** [04_primitive_types](exercises/04_primitive_types)
- **Issue:** All exercises did not compile.
- **Solution:**
  - primitive_types1: Defined is_evening boolean
  - primitive_types2: Tested the is_alphabetic() and is_numeric() methods with different characters
  - primitive_types3: Defined an array with 100 elements
  - primitive_types4: Defined new array with reference using &a and then range wanted: &variable_name[start_index..end_index]
  - primitive_types5: Deconstructed the tuple into two variables
  - primitive_types6: Accessed the tuple using tuple.0 , tuple.1 ... etc
- **Explanation:**
  - I think most of these problems/solutions are just learning Rust syntax, but I did do some exploration of the copying of tuple values.
- **Questions/Issues/Notes:**
  - First problem that inspired any actual testing was primitive_types5, where I tested what happened to the variables I created from deconstructing the tuple AFTER I altered the tuple values.  The variables did not change because they are copies of the original tuple values.

#### Exercise 05: Vectors

- **Exercise:** [05_vecs](exercises/05_vecs)
- **Issue:** Exercises don't compile, tests fail.
- **Solution:**
  - vecs1: Created a vector with the same entries as array 'a'.
  - vecs2: Created new vector after doing an operation on each entry of a different vector.  Multiplied each entry by 2.

- **Explanation:**
  - Basic vector operations in Rust. My first experience with programming with vectors.
- **Questions/Issues/Notes:**
  - Use Vectors when you need a dynamic, growable collection of elements of the same type.
  - Use Arrays when you need a fixed-size collection of elements of the same-type, and the size is known at compile time.
  - Use Tuples when you need to group a fixed number of elements of different types.
  - Rust vector formatting:

  ```rust
  let variable_name: Vec<type> = vec![elements];
  ```

### Week 3

#### Exercise 06: Move Semantics [06_move_semantics](exercises/06_move_semantics)

- **move_semantics1**
  - Issue: Originally the compiler did not like pushing to vec because it was immutable.
  - Solution:  Defined vec as mut (mutable).
  - Explanation: Vectors are immutable by default in Rust, as are variables. Need to explicitly say a variable can be changed or added to (declared mutable)

- **move_semantics2**
  - Issue: Originally the compiler did not like the initialization of vec1 gaining access to vec0.
  - Solution: Instead of fill_vec(vec0), I used a clone of vec0 in that method: fill_vec(vec0.clone())
  - Explanation: To make both vec0 and vec1 accessible at the same time, clone vec0 before passing it to the fill_vec function.

- **move_semantics3**
  - Issue: fill_vec wants to change vec, but the parameter vec is inputted as immutable by default.
  - Solution: The parameter also needs a mutable declaration if you want the function to change the parameter.
  - Explanation: In Rust, function parameters are immutable by default.

- **move_semantics4**
  - Issue: Doesn't compile because there are 2 mutable references to a single piece of data (x)
  - Solution: Move z's borrowing of x to after all operations of y are finished.
  - Explanation:  By reordering the lines there is only one mutable reference to x at any given time.

- **move_semantics5**
  - Issue: The functions defined parameter types do not match their parameter types in the main() implementation.  They handle ownership differently.
  - Solution: Matched the function parameter ownership type with their implementations.  get_char(data: &String) -> get_char(&data)
  string_uppercase(mut String) -> string_uppercase(data)
  - Explanation: parameters used in function calls need to match the way they were defined in the function.

- **Questions/Issues/Notes:**

  ```rust
  fn fill_vec(mut vec: Vec<i32>) -> Vec<i32>  VS. fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32>
  ```

  The first function is used to demonstrate ownership and modification, while the second function is used to demonstrate borrowing and modification WITHOUT transferring ownership.

#### Exercise 07: Structs [07_structs](exercises/07_structs)

- **structs1**
  - Issue: Both color structs had no defined fields. Needed a green struct defined for the tests. Also missing unit struct instantiation.
  - Solution: Added red, green, blue fields to ColorRegularStruct. Added 3 u8 fields to ColorTupleStruct. Defined test structs and instantiated unit struct.
  - Explanation: Structs can be defined with named fields with a data type, or as a tuple with just data types as parameters. Need to look up UnitStruct and its use.

- **structs2**
  - Issue: There is no order initialized that match what exists in the test.
  - Solution: The tests have unique values for the name and count fields, and then the rest of them match the defined template. Initialized am order struct which defined a name and count, and then copied the values of the template.
  - Explanation: to copy a field use: field_name: order_template.field_name. This is just a copy operation and still allows the template to be used again.

- **structs3**
  - Issue: The functions within the package do not have return types, nor defined operations.
  - Solution: Added return types to is_international (bool) and get_fees (u32). Also defined the function operations. NOTE: need to call self.field_name to access fields
  - Explanation: By implementing struct Package you can define functions that work exclusively for that struct. Reminds me of methods within in class in Java.

- **Questions/Issues/Notes:**
  - A unit struct is a struct with no fields. They are used as markers, zero-sized types, or for implementing traits.
  - Need to call self.field_name to access the struct fields. Functions within the struct Package, struct3.

#### Exercise 08: Enums [08_enums](exercises/08_enums)

- **enums1**
  - Issue: The Message enum has no types defined.
  - Solution: Added the enum Message types which were called in main()
  - Explanation: Enums are defined using the enum keyword followed by the name of the enum and a list of variants.

- **enums2**
  - Issue: The message enum has no types defined. The calls in main() have additional parameters beyond just a name.
  - Solution: Added all the variants with different variant data types that matched the calls in main().
  - Explanation: Enum variants can be a simple name, a tuple-like variant with associated data, or a struct-like variant with named fields.

- **enums3**
  - Issue: the impl State is missing instructions for the process function
  - Solution: Used the match keyword that allows comparison of a value against a series of patterns, and then executes code based on which pattern matches.
  - Explanation: The process function encapsulates the logic for handling different types of messages in one place. This makes the code more organized and easier to maintain. The process function uses pattern matching to determine which method to call based on the variant of the Message enum. This is a powerful feature in Rust that allows you to handle different cases in a concise and readable way.

- **Questions/Issues/Notes:**
  - enum variants can be a simple name, a tuple-like variant with associated data, or a struct-like variant with named fields.
  - You can implement methods for an enum using an `impl` block, just like you can for structs. This allows you to define behavior associated with the enum and its variants. By implementing methods for an enum, you can encapsulate functionality that operates on the enum and its variants.
  - By defining a process function, you can handle different types of messages in a consistent and organized manner, leveraging Rust's powerful pattern matching capabilities. This approach makes it easier to manage and extend the code as new message types are added.

#### Exercise 09: Strings [09_strings](exercises/09_strings)

- **strings1**
  - Issue: The function returns a String, but "blue" is a string slice
  - Solution: Added .to_string() method to the string slice: "blue".to_string()
  - Explanation: String and a string slice(&str) are different data types in Rust. Strings are owned, mutable, have dynamic size, and have unique methods. String slices are borrowed, immutable, and fixed size.

- **strings2**
  - Issue: is_a_color_word takes a string slice. word is a String. is_a_color_word(word) is a mismatched data type for the parameter.
  - Solution: Added & to the word parameter (&word) to create a reference to the string slice of the String word.
  - Explanation: Adding a reference to a String returns a string slice.  This is a borrowed reference and the original Sting is still accessible.

- **strings3**
  - Issue: All three methods are missing their operation definitions. They all have return types, so without operation instructions and a return of the proper type, this won't compile.
  - Solution: Used predefined methods which aligned with the function's intended use. For compose_me(), in order to concat I needed to convert to a String from the input parameter &str.
  - Explanation: trim() is defined for &str. Concatenation of a string only works for Strings. replace() works for both &str and String.

- **strings4**
  - Issue: The placeholder function has no operation, need to replace with defined functions string_slice() or string()
  - Solution: Replaced the placeholder function with the appropriate functions according to the output of the argument.
  - Explanation: Only placeholder replacement I got wrong was for the argument:

  ```rust
  placeholder(&String::from("abc")[0..1]);
  ```

  The range [0..1] operates on string slices but not Strings themselves. The range is not exclusive to &str, thus its documentation's output didn't help much the first attempt.

- **Questions/Issues/Notes:**
  - **Methods for Both `&str` and String**: `len`, `is_empty`, `contains`, `starts_with`, `ends_with`, `find`, `replace`, `trim`, `split`.
  - **Methods for `&str`**: `as_bytes`, chars, `lines`.
  - **Methods for String**: `push`, `push_str`, `insert`, `insert_str`, `clear`, `pop`.

### Week 4

#### Exercise 10: Modules [10_modules](exercises/10_modules)

- **modules1**
  - Issue: make_sausage() is being called in main, will not compile.
  - Solution: Declare make_sausage() function pub (public).
  - Explanation: make_sausage() is defined within a module and is private by default. Must be declared pub to use outside of the mod.

- **modules2**
  - Issue: delicious_snacks call in main cannot find fruit or veggie.
  - Solution: Added pub use self::function_name::path as new_name;
  - Explanation: By adding the line pub use self::function_name::path as new_name; within delicious_snacks creates a new path to an already defined const within the mod functions.

- **modules3**
  - Issue: cannot find value `UNIX_EPOCH` in the SystemTime module scope
  - Solution: Added `use std::time::{SystemTime, UNIX_EPOCH};`
  - Explanation: The line `use std::time::{SystemTime, UNIX_EPOCH};` brings the `SystemTime` struct and `UNIX_EPOCH` constant from the `std::time` module into the current scope. This allows you to use these items directly without needing to specify their full paths, making the code more readable and maintainable.

- **Questions/Issues/Notes:**
  - **`use` Keyword**: The `use` keyword is used to bring items from a module into scope, so you don't have to use the full path every time you refer to them.
  - **`as` Keyword**: The `as` keyword allows you to rename the imported item, which can help avoid naming conflicts or make the code more readable.

#### Exercise 11: HashMaps [11_hashmaps](exercises/11_hashmaps)

- **hashmaps1**
  - Issue: The compiler cannot find basket in the scope of the function fruit_basket().  basket not defined
  - Solution: Added basket definition as a new HashMap.  Inserted more fruits into basket. Don't forget to return basket (I did initially)
  - Explanation: For the tests to pass I needed to define at least five fruits in the fruit_basket() function

- **hashmaps2**
  - Issue: The tests want more fruits in the basket
  - Solution: I added more fruit enums, defined them as fruit_kinds, and then inserted them into the basket within the for loop (all qty 1)
  - Explanation: I'm really not sure of the teaching point of this exercise was, I may have done it wrong.  I added new Fruits directly to the enums and the fruit_kinds, and then added them to the basket via the for fruit in fruit_kinds loop.

- **hashmaps3**
  - Issue: the scores hashmaps did not have any entries.
  - Solution: added team names via scores.entry... and updated the scores via scores.get_mut(team_1_name).unwrap()...
  - Explanation:
    - **Adding Team Entries**: Used the `entry` method to insert teams if they do not already exist.
    - **Updating Scores**: Used the `get_mut` method to update the scores for each team.

- **Questions/Issues/Notes:**
  - Defining a HashMap within a function is not unique to Rust, but the way you manage and use hash maps differs between Rust and Java due to their respective language features and memory management models.
  - By using the `entry` method with `or_insert`, you can efficiently manage the contents of the HashMap and ensure that new fruits are only added if they are not already present.

### Week 6

#### Exercise 15: Traits

- **Exercise:** [15_traits](exercises/15_traits)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 16: Lifetimes

- **Exercise:** [16_lifetimes](exercises/16_lifetimes)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 17: Tests

- **Exercise:** [17_tests](exercises/17_tests)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

### Week 7

#### Exercise 18: Iterators

- **Exercise:** [18_iterators](exercises/18_iterators)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 19: Smart Pointers

- **Exercise:** [19_smart_pointers](exercises/19_smart_pointers)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 20: Threads

- **Exercise:** [20_threads](exercises/20_threads)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

### Week 8

#### Exercise 21: Macros

- **Exercise:** [21_macros](exercises/21_macros)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 22: Clippy

- **Exercise:** [22_clippy](exercises/22_clippy)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 23: Conversions

- **Exercise:** [23_conversions](exercises/23_conversions)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**
