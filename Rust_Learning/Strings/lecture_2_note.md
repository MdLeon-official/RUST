# Concatenation


### 🧠 What is Concatenation?
> **Concatenation** = Adding content to the **end of a string**.

---

### 🧱 String Types in Rust:
| Type        | Expandable? | Ownership | Example         |
|-------------|-------------|-----------|------------------|
| `String`    | ✅ Yes      | Heap owned | `String::from("Sylvester")` |
| `&str`      | ❌ No       | Borrowed   | `"Stallone"` (string literal) |

> To **modify a string**, use `String` (heap-allocated & mutable).

---

### 🧪 Example:
```rust
let mut full_name = String::from("Sylvester");
let last_name = "Stallone"; // &str

full_name.push(' ');        // Add a single character
full_name.push_str(last_name); // Add a &str
println!("{}", full_name);  // Output: Sylvester Stallone
```

---

### 🧩 Key Methods:

| Method       | Description                            | Accepts       |
|--------------|----------------------------------------|---------------|
| `.push(' ')` | Adds a **single character**            | `char`        |
| `.push_str()`| Adds a **string slice**                | `&str`        |

---

### ➕ Using the `+` Operator
```rust
let first_name = String::from("Sylvester");
let last_name = String::from("Stallone");

let full_name = first_name + &last_name;
// first_name is now **moved**
```

#### ⚠️ `+` Operator:
- Moves ownership of the **left String**
- Uses `.add()` method under the hood
- Right side must be `&str` (or `&String`, deref coerced)
- `first_name` becomes **invalid** after this

---

### 📦 If You Want to Keep the Original:
```rust
let full_name = first_name.clone() + &last_name;
```
- `clone()` makes a deep copy
- Now `first_name` is **still valid**, but **less efficient**

---

### ✅ Summary of 3 Concatenation Methods:
| Method        | Use When You...                        |
|---------------|----------------------------------------|
| `.push()`     | Add **1 character**                    |
| `.push_str()` | Add **&str / string literal**          |
| `+` operator  | Combine **2 strings** into a new one (ownership moves) |
