# Underscore in a Match Arm


#### ✅ **What is `match`?**
- A control flow operator used to compare a value against a series of patterns.
- Works like `if`/`else if` but is **cleaner**, more **readable**, and **exhaustive**.

---

### 🔄 **Refactored If/Else Using `match`**

**Old:**
```rust
if season == "summer" {
    println!("Hot and sunny!");
} else if season == "winter" {
    println!("Brr, so cold!");
} else {
    println!("Lots of rain!");
}
```

**Refactored with `match`:**
```rust
match season {
    "summer" => println!("Hot and sunny!"),
    "winter" => println!("Brr, so cold!"),
    _ => println!("Lots of rain!"),
}
```

---

### 📌 **Key Concepts**

- `match` checks patterns **top to bottom** and executes the **first matching arm**.
- `=>` is the "rocket syntax" used to associate patterns with code blocks.
- `,` separates each pattern arm (when not using `{}` blocks).
- `_` (underscore) is the **wildcard / catch-all** pattern — like an `else`.

---

### ⚠️ **Compiler Rules**

- All possible patterns must be covered.
  - For finite types like `bool`, you must list all values (`true`, `false`).
  - For infinite types like `String`, use `_` as a **fallback**.
- Each match arm must return the **same type**.

---

### 💡 **Common Mistake**
Do **not** place the `_` pattern at the top — it will match everything and make later arms **unreachable**.
