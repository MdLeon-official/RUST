## Reading vector elements:

### **Accessing Vector Elements**
1. **By Value**: Use the index with square brackets:
   ```rust
   let value = vector[index];
   ```
   - If the element type implements the `Copy` trait (e.g., `i32`), Rust creates a copy of the value.
   - If the type doesn’t implement `Copy` (e.g., `String`), ownership is moved, and the vector loses ownership of that element. Rust disallows partial ownership.

2. **By Reference**: Use the borrow operator (`&`):
   ```rust
   let reference = &vector[index];
   ```
   - Borrows a reference to the element without transferring ownership.
   - Works for all types, including heap-allocated types like `String`.

---

### **Ownership Rules**
- A vector owns its elements.
- When heap-allocated types (e.g., `String`) are inserted into a vector, ownership moves to the vector.
- Accessing an element by value for a type that doesn’t implement `Copy` will result in a compilation error:
   ```rust
   let value = vector[index]; // Error for `String`
   let reference = &vector[index]; // Correct
   ```

---

### **Runtime Errors**
- Accessing an invalid index (e.g., `vector[100]`) causes a **runtime panic** because:
  - The vector's length can change dynamically, so the compiler cannot guarantee validity at compile time.

---

### **Borrowing a Slice**
A slice is a reference to a sequence of elements:
- Syntax: `&vector[start..end]`
  - `start`: Inclusive.
  - `end`: Exclusive.
  ```rust
  let slice = &vector[1..3]; // Refers to elements at index 1 and 2
  ```
- Variations:
  - From a specific index to the end: `&vector[start..]`
  - From the start to a specific index: `&vector[..end]`
  - The entire vector: `&vector[..]`

---

### **Examples**
#### Vector of Integers (`Copy` trait):
```rust
let pizza_diameters = vec![8, 10, 12, 14];
let value = pizza_diameters[2]; // Copy: 12
let reference = &pizza_diameters[2]; // Reference: &12
```

#### Vector of Strings (Heap Allocated, Non-Copy):
```rust
let pepperoni = String::from("Pepperoni");
let mushroom = String::from("Mushroom");
let sausage = String::from("Sausage");
let pizza_toppings = vec![pepperoni, mushroom, sausage];

// Borrowing works:
let reference = &pizza_toppings[1]; // &String: "Mushroom"

// Slices:
let pizza_slice = &pizza_toppings[1..]; // ["Mushroom", "Sausage"]
```

#### Invalid Index Access:
```rust
let value = pizza_diameters[100]; // Runtime panic
```

---

### **Key Points**
- Use references (`&`) to avoid ownership issues.
- Always check indices to prevent runtime panics.
- Slices allow efficient partial access without copying or moving ownership.
