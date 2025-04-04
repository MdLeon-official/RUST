# Blocks in Functions


#### 🔹 What is a block?
- A block is a chunk of code inside `{}`.
- It defines a **new scope**, a local execution environment.

#### ✅ Syntax Example:

```rust
fn main() {
    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier  // No semicolon → this becomes the result
    };

    println!("Result: {}", calculation);  // Output: 27
}
```

---

### 🧠 Key Concepts

| Concept | Meaning |
|--------|---------|
| **Scope** | A region where variables live (e.g., a function or block). |
| **Block** | A `{}` section that defines a nested scope. |
| **Last line without semicolon** | Acts like a **return** from that block. |
| **Semicolon on last line** | Returns a **unit** `()` instead. |

---

### 📌 Notes:

- You can assign the **result of a block** to a variable.
- Variables declared **inside** the block go **out of scope** once the block ends.
- The block has access to **outer scope variables**.

---

### 💡 Why use this?
> Use blocks to group related logic without making a whole function.  
> It’s great for **temporary calculations**, **isolated logic**, or improving readability.
