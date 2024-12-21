# Rust Slices - A Comprehensive Guide

## 1. Intro to Slices

### What is a Slice?
A slice is like taking a piece of something bigger. It’s a reference to a part of a collection, like part of an array or string. It doesn’t take ownership of the original data, just borrows it.

### Examples of Slices:
- A string slice is a part of a string (like a few letters or words).
- An array slice is a part of an array (like some of its numbers or elements).

### How is it Like Borrowing?
Imagine a house:
- A full reference is like borrowing the whole house.
- A slice is like borrowing one room or one floor of the house.
- The owner keeps the house; you’re just borrowing a part of it.

### Special Thing About Slices:
- A slice can be just a part or even the entire thing!
- Example: You can borrow the whole house (or array) and still call it a slice.

### Why Use Slices?
- They let you work with part of the data without making a copy.
- They follow Rust’s rules about ownership, so your program stays safe.

---

## 2. Create a String Slice from a String

A string slice is a part of a string that you can borrow and use without owning it.

### How to Create a String Slice:
- To create a string slice, use `&` to borrow and `[start..end]` to specify the part you want.
  Example: `let first_name = &action_hero[0..6];`
- Slices use byte positions (`start..end`). English letters are 1 byte, but special symbols or emojis can be 2-4 bytes.
- Rust will panic if you go outside the string's size. Example: `&action_hero[0..50]` will cause an error if the string isn’t that long.

### Key Points:
- The type of a slice is `&str`, meaning it’s a reference to part of the string.
- Slices are useful because they save memory and let you work with only the part of the string you need.

---

## 3. String Slices and String Literals

### String Literals as String Slices:
A string literal is a fixed string value (like `"Arnold Schwarzenegger"`) embedded directly into the program at compile time. 

- String literals are stored in the binary of the program as a reference to a portion of text.
- Both string literals and references to String values are types of string slices (`&str`).

### Difference Between String Literals and String:
- In the previous example, the string `"Arnold Schwarzenegger"` was stored on the heap with the `String` type.
- In this case, the string is hard-coded in the binary and stored in read-only memory.

### Borrowing and Creating Slices:
You can borrow parts of a string literal just like you did with heap-allocated strings.

### Memory Management:
- The memory for string literals is guaranteed to stay in the program's memory because it is part of the binary.
- The slice reference (`&str`) will not become invalid even if the block of code that created it ends.

---

## 4. String Slice Lengths

### String Slice Length:
The length of a string slice is the number of bytes it occupies in memory, not the number of characters. Use the `len()` method to get the length in bytes.

### Example with Simple String:
- For the string slice `"pizza"`, it occupies 5 bytes because each English alphabet character is 1 byte.

### Creating Substrings (Slices):
You can create a smaller slice of a string using the `&` operator and slice syntax: `&food[0..3]`.

### Unicode and Multi-byte Characters:
- A single Unicode character, such as an emoji, might occupy more than one byte. For example, the pizza slice emoji 🍕 occupies 4 bytes, not 1 byte like regular characters.

### Problem with Slicing Multi-byte Characters:
- If you try to slice in the middle of a multi-byte character, Rust will panic at runtime.
- Example: Slicing `"🍕pizza"` as `&food[0..3]` will cause an error.

---

## 5. Syntactic Shortcuts

### Shortcut #1: Omitting the Start Index
- If you are slicing from the start of the string, you can omit the `0` in the slice syntax.
  Example: `&name[..6]` is the same as `&name[0..6]`.

### Shortcut #2: Slicing to the End
- If you want to slice from a specific index to the end of the string, you can omit the value after the `..`.
  Example: `&name[7..]` slices from byte 7 to the end of the string.

### Shortcut #3: Slicing the Entire String
- If you want the whole string as a slice, you can use an empty slice syntax: `..`.
  Example: `&name[..]` slices the entire string from the start to the end.

### Using Slices vs. References:
- Slice (`&str`) is a reference to a part of a string, while a reference to a `String` (`&String`) refers to the whole string.
- If you need the full string, it's easier to just use a reference (`&name`) instead of slicing.

---

## 6. String Slices as Function Parameters

### String Ownership vs. Borrowing:
- Passing a `String` to a function takes ownership of the string.
- Passing a `&String` borrows the full string, meaning it has a reference to the String.

### String Reference (`&String`):
- A function expecting a `&String` can only accept a reference to a full `String`.

### String Slice (`&str`):
- A function expecting a `&str` can accept both a string reference (`&String`) and a string slice (`&str`).
- String slices can be created from string literals or from a part of a `String` using slicing syntax.

### Deref Coercion:
- Rust has a feature called deref coercion that automatically converts a `&String` to a `&str`.
- Deref coercion does not work the other way around.

### Why `&str` is More Versatile:
- A function expecting a `&str` can accept both `&String` and `&str` because Rust can automatically convert `&String` to `&str`.
- A function expecting `&String` can only accept a reference to a full `String`.

---

## 7. Array Slices

### Array Slices:
An array slice is a reference to a portion of an array. It is created by borrowing a range of elements from the original array.

### Creating an Array Slice:
- The range in the square brackets defines the portion of the array you want to slice.
- The start index is inclusive, and the end index is exclusive.

### Common Range Examples:
- `&values[0..3]` — slices the array from index 0 to 3 (exclusive of 3).
- `&values[2..4]` — slices the array from index 2 to 4 (exclusive of 4).
- `&values[2..]` — slices the array from index 2 to the end of the array.
- `&values[..]` — slices the entire array.

### Full Array Reference vs Array Slice:
- A full array reference (e.g., `&[i32; 6]`) refers to the entire array with a fixed, specific length.
- An array slice (e.g., `&[i32]`) refers to a portion of the array with dynamic length.

### Function Parameter Flexibility:
Array slices are more flexible than full array references because they can represent any part of the array.

---

## 8. Deref Coercion with Array Slices

### Deref Coercion:
- Deref coercion allows Rust to automatically convert certain types when they are compatible.
- For instance, if a function expects a `&str`, Rust can automatically convert a `&String` into a `&str`. Similarly, this coercion applies to arrays and slices.

### Array References vs Array Slices:
- An array reference is a reference to the entire array with a fixed length (e.g., `&[i32; 6]`).
- An array slice is a reference to a portion of the array, not necessarily the entire array (e.g., `&[i32]`).

### Generalizing Array Parameters:
Using the array slice type (`&[i32]`) makes your function more flexible. It can accept both full array references and array slices without causing type mismatches.

---

## 9. Mutable Array Slices

### Mutable Slices of Arrays:
Unlike string slices, which are immutable by default, Rust allows mutable slices of arrays. This means you can modify parts of an array by borrowing a mutable reference to a slice of it.

### Array Setup:
You start by creating a mutable array. This is necessary because the array will be modified through the mutable slice.

### Creating Mutable Slice:
- To make a slice mutable, you add the `mut` keyword when borrowing the slice.

### Modifying Elements:
Once you have a mutable slice, you can modify the elements within that slice. Any changes made to the slice will directly affect the original array since the slice is just a reference to a portion of it.

### Effects on the Original Array:
Modifying an element in the slice will update that element in the original array, demonstrating that the slice is just a reference to the array's data.
