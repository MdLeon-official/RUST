# The Array Type


### Scalar Types:
- A scalar type holds a single value, such as integers, floats, Booleans, or characters.

### Compound Types:
- A compound type holds multiple values. It can represent a collection of multiple values of the same type.
- An **array** is a prime example of a compound type. It holds a fixed-size collection of homogeneous (same type) data.

### Arrays:
- Arrays in Rust store a sequence of values in order.
- They are **fixed size**, meaning once the array is defined, its size cannot change during runtime.
- The elements in an array are indexed starting from 0.
- An array is defined using square brackets `[]`, and its size is part of its type. For example, `i32` for integers and `6` for the array size would define a fixed-size array of 6 integers.

### Array Syntax:
- Example with integers:
  ```rust
  let numbers = [4, 8, 15, 16, 23, 42];
  ```
- To specify the type explicitly:
  ```rust
  let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];
  ```

### Key Points:
- Arrays in Rust are fixed-size. The compiler knows the exact number of elements at compile time.
- You cannot modify the array size once it's declared.
- An empty array must have its type specified because the compiler cannot infer its type or size.
- Arrays support methods like `length` to get the number of elements in the array.
  
Example of using `length`:
```rust
let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
println!("Length of apples array: {}", apples.len());
```

### Edge Case:
- An empty array requires type annotations to specify its intended type:
  ```rust
  let empty_array: [f64; 0] = [];
  ```

### Key Takeaway:
- Arrays are fixed-size and homogeneous (same data type for all elements). Understanding their structure is essential for working with collections in Rust.
