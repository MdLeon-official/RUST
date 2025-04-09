

# String Slices

#### 🧵 What is a String Slice?
A **string slice** is a reference (`&`) to a **part** of a string (not the whole thing).  
Its type is `&str`.

#### 🍕 Reminder:
> 🔹 `&String` = full string reference  
> 🔹 `&str` = string slice (a part)

---

#### 🧪 Example:
```rust
let action_hero = String::from("Arnold Schwarzenegger");

let first_name = &action_hero[0..6]; // "Arnold"
let last_name = &action_hero[7..21]; // "Schwarzenegger"
```

#### 🔍 How it works:
- The syntax: `&string_variable[start..end]`
- It borrows **bytes**, not characters.
- In `"Arnold"`, each letter = 1 byte, so 0..6 means byte 0 to 5.
- ⚠️ Rust will **panic** if you try to slice out of bounds!

---

#### ⚠️ Remember This:
> 🧠 `&str` = string slice = a piece of the string  
> 📏 Uses **byte index**, not visual characters  
> ❌ Going past the string size = **runtime error**
