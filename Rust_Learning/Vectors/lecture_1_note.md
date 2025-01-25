## Create a vector: 

#### What Are Vectors?
- **Vector**: A flexible data structure in Rust that is similar to an array but can grow or shrink in size dynamically.
- **Array Limitation**: Arrays have a fixed size and cannot grow or shrink. Use a vector if you need dynamic resizing.

#### Creating an Array
- Arrays are written with square brackets `[]` and contain elements of the same type.
  ```rust
  let arr = ["item1", "item2", "item3"];
  ```
- Arrays have a fixed size. For example:
  ```rust
  let arr: [&str; 3] = ["item1", "item2", "item3"];
  ```
  - `&str`: Type of the elements (string slices).
  - `3`: Length of the array (fixed).

#### Introducing Vectors
- **Key Advantage**: Vectors can change size during runtime, making them ideal when the number of elements isn't fixed.
- **Memory**: Vectors are stored on the heap, enabling dynamic resizing.

#### Creating a Vector
1. **Empty Vector**:
   ```rust
   let vec: Vec<i32> = Vec::new();
   ```
   - Use `Vec::new()` to create an empty vector.
   - You must specify the type of elements (`Vec<i32>` for integers).

   **Alternative Syntax**: Use the "turbofish" operator:
   ```rust
   let vec = Vec::<i32>::new();
   ```

2. **Populated Vector** (Using `vec!` Macro):
   ```rust
   let vec = vec![8, 10, 12, 14];
   ```
   - `vec!` initializes a vector with predefined values.
   - If the values are known, Rust infers the type, so type annotations are optional.

#### When to Use Arrays vs. Vectors
- Use an **array** if:
  - The size of the collection is fixed and known in advance.
  - Example: Storing days of the week.
- Use a **vector** if:
  - The size of the collection might grow or shrink at runtime.
  - Example: Asking a user for a list of file names (unknown size).

#### Example: Arrays vs. Vectors
```rust
// Array: Fixed size
let fruits = ["apple", "banana", "cherry"]; // Length is 3.

// Vector: Dynamic size
let mut numbers = vec![1, 2, 3];
numbers.push(4); // Adds 4 to the vector.
numbers.pop();   // Removes the last element.
```

#### Debugging Vectors
- Vectors implement the `Debug` trait, allowing easy printing with `{:?}` or `{:#?}`.
  ```rust
  println!("{:?}", vec![1, 2, 3]); // Outputs: [1, 2, 3]
  ```

