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

- **Alternative Approaches:**
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
  - Explanation:

- **Alternative Approaches:**
- **Questions/Issues:**
  - A unit struct is a struct with no fields. They are used as markers, zero-sized types, or for implementing traits.
  - Need to call self.field_name to access the struct fields. Functions within the struct Package, struct3.

#### Exercise 08: Enums

- **Exercise:** [08_enums](exercises/08_enums)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

### Week 4

#### Exercise 09: Strings

- **Exercise:** [09_strings](exercises/09_strings)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 10: Modules

- **Exercise:** [10_modules](exercises/10_modules)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 11: HashMaps

- **Exercise:** [11_hashmaps](exercises/11_hashmaps)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

### Week 5

#### Exercise 12: Options

- **Exercise:** [12_options](exercises/12_options)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 13: Error Handling

- **Exercise:** [13_error_handling](exercises/13_error_handling)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

#### Exercise 14: Generics

- **Exercise:** [14_generics](exercises/14_generics)
- **Issue:**
- **Solution:**
- **Explanation:**
- **Alternative Approaches:**
- **Questions/Issues:**

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
