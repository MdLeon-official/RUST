# Ownership with Immutable and Mutable References


In Rust:
- Types that implement the `Copy` trait are *cheap to copy* (like integers).
- When a variable is assigned to another, the **value is copied**, not moved.
- That means *both* variables are valid and usable.

---

### 🔹 Immutable References: `&T`

- `&T` (an immutable reference) **implements** the `Copy` trait.
- So, if you write:

  ```rust
  let coffee = String::from("Mocha");
  let a = &coffee;
  let b = a;
  println!("{a} and {b}");
  ```

  It works perfectly.

- Why?
  - `a` is an immutable reference (`&String`).
  - `b = a` creates a **copy** of that reference.
  - Both `a` and `b` are valid — they both point to the same data (`"Mocha"`) on the heap.
  - This is **safe** because neither of them can modify the data.

> ✅ Immutable references are cheap (just a pointer), safe, and allow multiple copies.

---

### 🔸 Mutable References: `&mut T`

- `&mut T` **does NOT** implement the `Copy` trait.
- So, if you try:

  ```rust
  let mut coffee = String::from("Mocha");
  let a = &mut coffee;
  let b = a;  // 👈 move occurs here!
  println!("{a}");
  ```

  ❌ You get a **compiler error**:  
  `move occurs because 'a' has type '&mut String' which does not implement the Copy trait`

- Why?
  - You can only have **one** mutable reference to data at a time.
  - When you write `let b = a;`, ownership of the mutable reference moves from `a` to `b`.
  - After the move, `a` is invalid — trying to use it causes an error.

> ❌ Mutable references **cannot be copied** because it would break Rust’s safety guarantees (two mut refs = data races).

---

### 🧠 How to Remember:

- ✅ `&T` (immutable ref) = Copyable, multiple allowed.
- ❌ `&mut T` (mutable ref) = **Not** Copyable, only one allowed.

---

### 📌 Bonus Tip:

Ownership of a reference moves **only when** it’s a mutable reference and you're assigning it or passing it somewhere.  
You can still use it **before** it moves:

```rust
let mut coffee = String::from("Mocha");
let a = &mut coffee;
println!("{a}"); // ✅ still valid
let b = a;       // ownership moves here
// println!("{a}"); ❌ not valid anymore
```
