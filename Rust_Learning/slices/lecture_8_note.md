# Deref Coercion with Array Slices



### ✅ Array Reference vs. Array Slice

| Concept | Syntax | Type | Notes |
|--------|--------|------|-------|
| **Full array reference** | `&array` | `&[T; N]` | Reference to an array of fixed length. Length is part of the type. |
| **Array slice** | `&array[start..end]` | `&[T]` | Reference to a subset (or full set) of the array. Length is *not* part of the type. |

---

### 📌 Why Slices Are Preferred for Functions

#### Less Flexible:
```rust
fn print_length(reference: &[i32; 6]) { ... }
```
- Only accepts a reference to a 6-element array.
- `&[i32; 5]` or a slice like `&array[0..3]` will cause a type mismatch.

#### More Flexible:
```rust
fn print_length(reference: &[i32]) { ... }
```
- Accepts **any** slice — `&[i32]`, `&array`, `&array[1..4]`, etc.
- Works with both fixed-length references and slices due to **deref coercion**.

---

### 🧠 How Deref Coercion Works

If a function expects a `&[i32]` (a slice), and you pass `&[i32; 6]` (a full array reference), Rust **automatically coerces** the full reference to a slice:
```rust
let array = [1, 2, 3, 4, 5, 6];
let reference = &array;
print_length(reference); // OK! deref coercion to &[i32]
```

---

### 🧪 Summary Tips

- Use `&[T]` in function parameters unless you *really* need to enforce a specific array length.
- Slices are dynamically sized — this gives flexibility and is idiomatic in Rust.
- Array references (`&[T; N]`) are strict and include the length in the type — use sparingly.
- The same idea applies to `String` and `&str` as it does to `[T; N]` and `&[T]`.
