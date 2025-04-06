# Ownership with Arrays and Tuples


### 🔑 Core Idea:
Ownership **cascades down**:
- A **variable** owns the **array/tuple**, and
- The array/tuple owns the **elements** inside it.

---

### 📗 Arrays with Scalar Types (e.g., `bool`, `i32`, etc.)

```rust
let registrations = [true, false, true];
let first = registrations[0]; // This works!
```

- `bool` implements the `Copy` trait.
- So `first` gets a **copy**.
- `registrations` still owns the array and its elements.

✅ You can still use both `registrations` and `first`.

---

### 📘 Arrays with Heap Types (e.g., `String`)

```rust
let languages = [String::from("Rust"), String::from("JavaScript")];
let first = languages[0]; // ❌ ERROR! Move occurs.
```

- `String` **does NOT** implement `Copy`.
- Accessing `languages[0]` **tries to move ownership**.
- Partial ownership isn’t allowed → **compile error**.

✅ Solutions:
```rust
let first = languages[0].clone(); // ✅ Makes a full copy
let first = &languages[0];        // ✅ Borrow it (reference)
```

---

### 📙 Tuples

```rust
let registrations = (true, false, true);
let first = registrations.0; // ✅ Works, bool is Copy
```

```rust
let languages = (String::from("Rust"), String::from("JavaScript"));
let first = &languages.0; // ✅ Works with borrow
// let first = languages.0; // ❌ ERROR, move occurs
```

Same rules apply as arrays:
- Tuple owns its elements.
- Borrowing or cloning required for non-Copy types.

---

### 💡 Tip:
Use `.clone()` if:
- You **want to copy** heap data.

Use `&` (borrow) if:
- You **just need access**, not ownership.
- Want to avoid unnecessary memory use.
