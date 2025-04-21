

## 🧵 Rust Strings

### 🧠 What is a String?

A **string** is just **text**, made of characters.

Rust has **two main string types**:

| Type        | Name      | Stored in | Mutable? | Owns Data? | Notes                          |
|-------------|-----------|-----------|----------|------------|--------------------------------|
| `&str`      | str slice | Binary (code) | ❌ No    | ❌ No       | Borrowed, read-only            |
| `String`    | Heap      | Heap      | ✅ Yes   | ✅ Yes      | Growable, modifiable text      |

---

### 🔍 Examples

```rust
let pirate = "Bloodhook";       // &str (string slice, read-only)
let sailor = String::from("Bloodhook"); // String (heap allocated, modifiable)
let bad_guy = pirate.to_string(); // Also creates a String
```

- `String::new()` ➝ creates an **empty** String
- `String::from("text")` ➝ make a **String from &str**
- `some_str.to_string()` ➝ also turns &str into a heap String

---

### 🧷 Important Difference

- `&str` is like borrowing a **read-only label** stuck to the wall.
- `String` is like your own **notebook** – you can write and erase.

---

### ❗ Indexing in Strings

❌ Rust **doesn't allow** this:

```rust
let first = pirate[0]; // ❌ Will NOT work
```

Why?

- Rust strings are **UTF-8**, not every character = 1 byte.
  - `A` = 1 byte ✅  
  - `😎` = 4 bytes ❗
- Indexing by number (like `[0]`) targets **bytes**, not characters.

---

### ✅ Safe Way to Get Part of a String

Use a **range**:

```rust
let first = &pirate[0..1]; // ✅ Gets the "B"
```

- Syntax: `[start..end]` (end is not included)
- Safer because you’re telling Rust how **many bytes** to take.

---

### 🧠 Remembering Tip

- Think: `&str` is like **a poster on the wall** – you can look, not touch.
- `String` is like **your personal diary** – you can write and rip pages.
- Don’t use `[n]` on strings – **Rust protects you** from breaking emojis 🫣.
