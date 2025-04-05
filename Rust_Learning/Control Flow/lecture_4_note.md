# Assigning Result of if Statement to Variable


You can use an `if`/`else` expression **as a value**, and assign its result to a variable.

---

### 📦 Example:

```rust
fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("The number is {}", result);
}

fn main() {
    even_or_odd(17);  // Output: The number is odd
    even_or_odd(100); // Output: The number is even
}
```

---

### 💡 Key Concepts:

- `if`/`else` in Rust **returns a value**.
- Both blocks **must return the same type** (in this case, a string slice `&str`).
- You can directly assign the result to a variable with `let`.

---

### 🧠 Think of it like Rust’s ternary alternative:

```rust
let result = if condition { value_if_true } else { value_if_false };
```

There’s **no ternary `? :` operator** in Rust — use `if`/`else` expressions instead.

---

### ⚠️ Remember:
- The `if` and `else` blocks **must return compatible types**.
- Don’t forget the semicolon (`;`) at the end of the assignment.
- You can still split over multiple lines for readability.
