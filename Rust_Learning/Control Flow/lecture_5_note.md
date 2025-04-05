# The match Statement


#### 📌 What is `match`?
- Works like `switch` in other languages.
- Compares a value against multiple possible patterns (called *arms*).
- Forces you to **handle all possibilities**, making code safer.

#### 🧠 Why use `match`?
- Cleaner than multiple `if` / `else if`.
- Rust compiler checks all possible variants are handled (exhaustive checking).
- Can **return values** directly from arms.

---

### 🧪 Example: Boolean Match

```rust
let evaluation = true;

let value = match evaluation {
    true => 20,
    false => 40,
};

println!("Value is: {}", value);
```

- `evaluation` is either `true` or `false`.
- `match` ensures both values are handled.
- Each arm returns a value (`20` or `40`) assigned to `value`.

#### 💡 Notes:
- Use `=>` to define the result of a match arm.
- Curly braces `{}` are optional **if returning a single value**.
- Commas **are required** when arms return values.

---

### ⚠️ Compiler Enforces Exhaustiveness
If you forget a possibility:
```rust
match evaluation {
    true => 20,
}
```
Rust will throw an error: **"missing match arm: false not covered"**

---

### 🟢 `match` is Good for:
- Booleans ✅
- Enums ✅
- Integers ✅
- Strings ✅
- Tuples, arrays, custom types ✅
