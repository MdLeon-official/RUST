# String Slices and String Literals


#### 🔹 **String Literal**
- Written in double quotes (`"Arnold Schwarzenegger"`).
- Stored **in the final executable**.
- Available **for the entire runtime** of the program.
- Type: `&'static str` — a **string slice** with `'static` lifetime.

```rust
let action_hero = "Arnold Schwarzenegger"; // &'static str
```

---

#### 🔹 **String Slice (`&str`)**
- A **reference** to some portion of a string.
- Can point to:
  - A full string (like the entire `"Arnold Schwarzenegger"`)
  - A **subset** (like just `"Arnold"` or `"Schwarzenegger"`)

```rust
let first_name = &action_hero[0..6]; // "Arnold"
```

> ✅ `&str` is always **immutable** and doesn't own the data.

---

#### 🧠 **Important Notes**
- All of these are string slices:
  ```rust
  let action_hero = "Arnold Schwarzenegger";  // &str
  let first_name = &action_hero[0..6];        // &str
  let last_name  = &action_hero[7..];         // &str
  ```

- When you do:
  ```rust
  let first_name = {
      let action_hero = "Arnold Schwarzenegger";
      &action_hero[0..6]
  };
  ```
  - This is **NOT** a dangling reference ❌.
  - The string literal lives **for the entire program**, not just the block.
  - `first_name` gets its **own reference** to a portion of static memory.

---

#### 🔄 **Difference from Heap `String`**
| **Heap String**               | **String Literal**             |
|------------------------------|-------------------------------|
| `String` owns its data       | String literal is embedded    |
| Lives on the heap            | Lives in executable (static)  |
| Needs `.as_str()` to slice   | Already a `&str`              |

---

#### 💡 **Key Takeaway**
> Any `"string in double quotes"` is a **string slice** (`&'static str`) that points to **read-only memory**, valid throughout the program’s lifetime.
