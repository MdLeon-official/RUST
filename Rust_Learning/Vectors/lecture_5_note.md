## Ownership with Vectors:

1. **Ownership Transfer**:
   - Assigning a vector to a new variable transfers ownership.  
     Example:
     ```rust
     let pizza_toppings = vec!["Pepperoni", "Mushrooms", "Sausage"];
     let delicious_toppings = pizza_toppings; // Ownership moves
     // `pizza_toppings` can no longer be used
     ```
   - Using the original variable after ownership transfer results in a compile-time error.

2. **Borrowing Rules**:
   - **Mutable References**:
     - Only one mutable reference is allowed at a time.
   - **Immutable References**:
     - Multiple immutable references are allowed simultaneously.
   - **Mutable + Immutable References**:
     - A mutable reference cannot coexist with any immutable references.

---

### Vector Memory Reallocation
- **Behind the scenes**:
  - Adding/removing elements may require the vector to reallocate memory on the heap.
  - This can cause references to the vector's elements to become invalid if they point to the old heap location.

---

### Examples and Compiler Behavior
#### Immutable Borrow with No Conflict
```rust
let pizza_toppings = vec!["Pepperoni", "Mushrooms", "Sausage"];
let topping_reference = &pizza_toppings[1]; // Immutable borrow
println!("The topping is {}", topping_reference); // Works fine
```

#### Immutable Borrow + Mutable Borrow (Conflict)
```rust
let mut delicious_toppings = vec!["Pepperoni", "Mushrooms", "Sausage"];
let topping_reference = &delicious_toppings[1]; // Immutable borrow

delicious_toppings.push("Olives"); // Mutable borrow causes conflict
// println!("The topping is {}", topping_reference); // Compile error
```
**Error**: Rust prohibits the mutable borrow (`push`) because it could move the vector in memory, invalidating `topping_reference`.

---

### Resolving Conflicts
- **Use the immutable reference before the mutable one:**
  ```rust
  let mut delicious_toppings = vec!["Pepperoni", "Mushrooms", "Sausage"];
  let topping_reference = &delicious_toppings[1];
  println!("The topping is {}", topping_reference); // Immutable reference used first
  delicious_toppings.push("Olives"); // Mutable reference used after
  ```
- **Use the mutable reference before the immutable one:**
  ```rust
  let mut delicious_toppings = vec!["Pepperoni", "Mushrooms", "Sausage"];
  delicious_toppings.push("Olives"); // Mutable borrow used first
  let topping_reference = &delicious_toppings[1];
  println!("The topping is {}", topping_reference); // Immutable reference used after
  ```

---

### Key Concepts
- **Lifetimes**:
  - Rust tracks how long references are valid and ensures no conflicts between mutable and immutable references within their overlapping lifetimes.
- **Safety**:
  - Rust prevents references from pointing to invalid data, ensuring memory safety without runtime checks.

