# Scope and Ownership


- A **block** is a chunk of code between `{ }`.
- A **scope** is the area in which a variable is valid.
- When a block starts, a new scope begins.
- When a block ends, all variables inside it go **out of scope** and are **dropped** (removed from memory).

---

### 🔑 **Ownership Basics (Stack Version)**

- Every **value** in Rust has an **owner**.
- The **owner** is usually the variable that holds the value.
- Example:
  ```rust
  let age = 33;
  ```
  - Here, `age` is the owner of the value `33`.

- When the **owner goes out of scope**, the value is:
  - **Removed from memory**.
  - If it’s on the **stack**, it’s **popped off**.

---

### 📦 **Nested Blocks Example**
```rust
fn main() {
    let age = 33; // scope: main function

    {
        let is_handsome = true; // scope: inner block
        println!("{}", is_handsome);
    } // 'is_handsome' is dropped here

    println!("{}", age); // valid here
}
```
- `is_handsome` exists **only** inside the inner block.
- You **can't use it outside** because it’s dropped when the inner block ends.

---

### 🔁 **Stack Cleanup: LIFO (Last In, First Out)**

- Stack works like a **pile of plates**: last thing added gets removed first.
- If you do this:
  ```rust
  let age = 33;
  let is_handsome = true;
  ```
  - `is_handsome` is **popped first**.
  - Then `age` is **popped**.

---

### 🧹 **Summary**

- Each value has an **owner** (usually a variable).
- When the **owner goes out of scope**, the value is **dropped**.
- This makes sure Rust manages memory **automatically and safely**.
- So far, this is with **stack data**. Heap comes later.
