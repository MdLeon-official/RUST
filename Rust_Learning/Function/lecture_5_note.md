# The Unit as a Return Type


#### ✅ What is it?
- A **unit** is an **empty tuple**: `()`
- It **holds no data**, but **is still a value**.
- Type and value are written the same way: `()`

---

### 📦 Tuple Reminder:

```rust
let t = (5, true, "hello");
// Type: (i32, bool, &str)
```

Unit is just:
```rust
let u = ();  // Type: ()
```

---

### 🔁 **Functions and Unit Return Type**

- If a function **does not return anything**, it **implicitly returns a unit** `()`.

#### 🧪 Example:

```rust
fn mystery() {
    // does nothing
}
```

- Equivalent to:

```rust
fn mystery() -> () {
    // does nothing
}
```

---

### 🔔 Key Points:

✅ If **no `return`** and **no last line value**, Rust **defaults** return to `()`  
✅ Even if the function **only prints** or ends in `;`, it returns `()`  

```rust
fn mystery() {
    println!("Hello there");  // ends with ; → no value → returns ()
}
```

---

### 💬 Summary:
> **Every function in Rust returns something.**  
> If nothing is returned explicitly or implicitly, the return is `()` by default.
