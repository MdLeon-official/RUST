/*
Rust Data Types: A Simple Guide

In Rust, every value has a **data type**, which tells the compiler what kind of data you're working with. Rust's data types can be divided into **scalar types** and **compound types**.

---

1. Scalar Types
Scalar types represent a single value.

a. Integers
- Whole numbers, both positive and negative.
- Examples: `1`, `-42`, `255`
- Common types: 
  - Signed: `i8`, `i16`, `i32`, `i64`, `i128` (allows negatives)
  - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128` (only positive)
  - Default: `i32`

b. Floating-Point Numbers
- Numbers with decimals.
- Examples: `3.14`, `-2.718`
- Types: `f32` (32-bit), `f64` (64-bit, more precise)
- Default: `f64`

c. Boolean
- Represents `true` or `false`.
- Example: `let is_rust_fun: bool = true;`

d. Character
- A single character enclosed in single quotes.
- Supports Unicode, so you can use emojis and non-English characters.
- Example: `let letter: char = 'A';`

---

2. Compound Types
Compound types group multiple values into one type.

a. Tuples
- A collection of values of different types.
- Fixed size; you can’t add or remove elements.
- Example:
  ```rust
  let person: (i32, f64, char) = (25, 70.5, 'M');
  let (age, weight, gender) = person; // Destructure
  println!("{}", person.0); // Access by index
*/

//TRAITS:
//a trait is a contract that requires that a type support one or more methods

//THE DISPLAY TRAITS
// In Rust, the Display trait is used to define how something should look when
//  printed nicely for people to read, like with the {} placeholder in println!.
//  It’s for making custom, clean text output for your types.


//THE DEBUG TRAIT
//The Debug trait in Rust allows you to print detailed, developer-friendly 
// information about an object, typically using the {:?} formatter for debugging 
// purposes.

fn main() {
    let season = ["spring", "winter", "summer", "rain"];
    println!("{:?}",season);
    println!("{:#?}",season);

    //The dbg! macro
    dbg!(2+2);
    dbg!(season);

    //The tuple types
    let employee = ("Leon", 18, "RUST");
    // let name =  employee.0;
    // let id =  employee.1;
    // let position =  employee.2;

    let (name, id, position) = employee;
    println!("Name: {name} ID: {id} Position: {position}");
    println!("{employee:#?}");
    
    //RANGES
    let _number = 1..31;
    let alphabet = 'a'..='z';
    for i in alphabet{

        println!("{}", i);
    }

    //Generic in rust
    // Generics in Rust let you write code that works with any type, 
    // using a placeholder for the type that you decide later.
    let _n:std::ops::Range<i8> = 1..31;
    let _c:std::ops::Range<char> = 'a'..'z';

}
