# The format! Macro


### 🔸 What is `format!`?
- `format!` is like `println!`, but **it returns a `String` instead of printing**.
- It's used when you want to build a string and **store or return** it, not just display it.

---

### 🧪 Syntax Example:
```rust
let icon = format!("{} {}", first_name, last_name);
```
- This will **combine** `first_name` and `last_name` with a space in the middle.
- Returns a **heap-allocated String**.

---

### 🔄 Ownership:
- **Does NOT take ownership**.
- Just like `println!`, it **borrows** values behind the scenes.
- So you can still use `first_name` and `last_name` after.

---

### 🧩 Interpolation Styles:
```rust
// Normal style
format!("{} {}", first_name, last_name);

// Positional
format!("{0} {1} {0}", first_name, last_name); 
// Output: "Sylvester Stallone Sylvester"
```

---

### 🔣 Works With:
- Anything that implements the **`Display` trait** (strings, numbers, floats, etc.)
- Output is always a `String` (you can store it in a variable or return it from a function)

---

### ✅ When to Use:
- When you need a **dynamic string** (with variables/values inside)
- When you want to **store** or **return** the string (not just print)
