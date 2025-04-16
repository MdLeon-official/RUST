# The String Type


### 📌 Two Main String Types in Rust
| Type | Example | Stored In | Mutable? |
|------|---------|-----------|----------|
| `str` (string slice) | `"pasta"` | Embedded in the binary | ❌ No |
| `String` | `String::from("KitKat")` | Heap | ✅ Yes |

---

### 📙 `str` – The Simple One
- This is what you get when you write a string in double quotes (`"hello"`).
- It's **known at compile time**, **not mutable**, and lives **inside the binary**.
- Example:
  ```rust
  let food = "pasta"; // This is a &str
  ```

---

### 📙 `String` – The Heap One
- This is a **dynamic** string stored on the **heap**.
- It can **change**, **grow**, and **shrink**.
- Useful when:
  - Taking user input
  - Reading from a file
  - Modifying the string content

---

### 🔨 Creating a `String`
**1. Using `String::new()`** – creates an empty string.
```rust
let text = String::new();
```

**2. Using `String::from("text")`** – creates a `String` from a `&str`.
```rust
let candy = String::from("KitKat");
```

---

### 👑 Ownership & Cleanup
- The variable that holds the `String` is the **owner**.
- When that variable goes **out of scope**, it automatically **cleans up** the heap memory.

```rust
fn main() {
    let candy = String::from("KitKat");
    // 'candy' owns the String. When main ends, it's cleaned up.
}
```

---

### 🧠 Key Takeaways
- Use `str` for **fixed, known-at-compile-time** text.
- Use `String` for **dynamic, changeable** text.
- `String` lives on the **heap**, so it can grow or shrink.
- `String::new()` → empty string  
  `String::from("text")` → string with content
- The **owner variable** is responsible for memory cleanup.
