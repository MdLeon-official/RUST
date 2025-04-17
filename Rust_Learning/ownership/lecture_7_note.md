# Moves and Ownership


### 🔑 Key Concepts

- **Ownership**: In Rust, each value has **one owner** at a time.
- **Move**: When you assign a value from one variable to another, **ownership moves**. The original variable becomes invalid.

---

### ✅ Stack Data (like integers)

```rust
let time = 2025;
let year = time;
```

- Integers implement the `Copy` trait → A **duplicate** is created.
- `time` and `year` both valid. No move happens.

---

### ❌ Heap Data (like `String`)

```rust
let person = String::from("Boris");
let genius = person;
```

- `String` does **not** implement the `Copy` trait → No duplicate.
- Only the **stack part** (pointer, length, capacity) is copied.
- **Heap data** is not cloned.
- Ownership **moves** to `genius`.
- `person` becomes **invalid**. Cannot use `person` anymore.

```rust
println!("{}", person); // ❌ Error: "value borrowed after move"
```

---

### 🔥 Why Move Matters

- Prevents **double free errors** (where two variables try to free the same memory).
- Only one variable owns the heap data → Safer memory management.

---

### 📌 Summary

| Action                          | Heap Data (`String`)         | Stack Data (`i32`, `bool`) |
|-------------------------------|------------------------------|----------------------------|
| Assignment to another variable | Ownership moves              | Copy happens               |
| Original variable usable?     | ❌ No, becomes invalid        | ✅ Yes                     |
| Memory safety                 | ✔ Prevents double free error | Not an issue               |
