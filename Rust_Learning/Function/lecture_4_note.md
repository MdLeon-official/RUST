# Implicit Return Values


#### ✅ What is it?
- Rust **automatically returns** the **last expression** in a function **if no `return` keyword** is used.
- No **semicolon (;)** → value is **returned**
- With **semicolon** → Rust treats it as a **statement**, not a return value.

---

#### 🔁 Example:

```rust
fn square(number: i32) -> i32 {
    number * number  // ✅ Implicit return (no semicolon)
}
```

Same as:
```rust
fn square(number: i32) -> i32 {
    return number * number;
}
```

---

#### ⚠️ Gotcha:

```rust
fn square(number: i32) -> i32 {
    number * number;  // ❌ Ends as a statement, returns ()
}
```

To force return of a specific value:
```rust
fn square(number: i32) -> i32 {
    number * number;  
    2                 // ✅ Returns 2 (last line without ;)
}
```

---

#### 🧠 Tip:
- Think of **semicolon** as a **period**: ends a thought.
- No semicolon = “I want to return this”.
