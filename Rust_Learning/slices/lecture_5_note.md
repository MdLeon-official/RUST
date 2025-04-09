# Syntactic Shortcuts


### 🔹 1. **Start from 0** → Leave it out

Instead of writing `&s[0..6]`, just write:

```rust
let slice = &s[..6]; // Start from beginning to byte 6 (exclusive)
```

✅ Equivalent to `&s[0..6]`

---

### 🔹 2. **To the End** → Omit the end index

Instead of writing `&s[7..21]`, just write:

```rust
let slice = &s[7..]; // Start at byte 7 to the end
```

✅ Dynamic and adapts to string length changes

---

### 🔹 3. **Full Slice** → Omit both start and end

If you want the entire string as a slice:

```rust
let full = &s[..]; // From start to end
```

✅ Equivalent to `&s[0..s.len()]`

---

### 📝 Notes on Types

| Expression      | Type            | Meaning |
|----------------|-----------------|---------|
| `&s`            | `&String`        | A reference to the whole `String` |
| `&s[..]`        | `&str`           | A slice referencing the full content |
| `&s[..6]`       | `&str`           | A slice referencing the first 6 bytes |
| `&s[7..]`       | `&str`           | A slice from byte 7 to end |

---

### ⚠️ Reminder

- These shortcuts are helpful when:
  - You're working with dynamic content.
  - You want cleaner, more maintainable code.
- All slices are based on **byte positions**, not character positions. Be cautious with multi-byte characters like emojis (as discussed in the previous lesson).

---

### ✅ Example

```rust
let action_hero = String::from("Arnold Schwarzenegger");

let first_name = &action_hero[..6];      // "Arnold"
let last_name  = &action_hero[7..];      // "Schwarzenegger"
let full_name  = &action_hero[..];       // "Arnold Schwarzenegger"

println!("First: {}", first_name);
println!("Last: {}", last_name);
println!("Full: {}", full_name);
```
