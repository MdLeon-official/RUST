# String, &String, str, and &str


### 🔹 `String`
- **Heap-allocated**, growable text.
- Created at **runtime**, e.g. `let s = String::from("Hello");`
- Use when text is **dynamic** (changes, unknown in advance).

### 🔹 `&String`
- **Reference** to a `String`.
- Borrowed, points to heap-allocated string.
- Saves memory (no duplication), e.g. `let s_ref = &s;`

---

### 🔹 `str`
- A **string slice**, like `"Hello"` in code.
- **Hardcoded** text, **readonly**, known at **compile time**.
- Embedded **directly in the binary** (not stack/heap).
- Rarely used directly. You always interact with `&str`.

### 🔹 `&str`
- **Reference** to a string slice (`str`).
- Most common type when using **string literals**:  
  e.g. `let greeting = "Hello"; // type is &str`
- Points to the memory in the binary where the text is stored.
- No copying, just referencing static memory.

---

### 💡 Summary Table

| Type      | Stored in   | Mutable? | Owned? | When to Use                     |
|-----------|-------------|----------|--------|---------------------------------|
| `String`  | Heap        | ✅       | ✅     | When you need dynamic text      |
| `&String` | Heap (ref)  | ❌       | ❌     | When borrowing a `String`       |
| `str`     | Binary      | ❌       | ❌     | Rarely used directly            |
| `&str`    | Binary (ref)| ❌       | ❌     | Most common for fixed text      |

---

### 🎯 Tips to Remember
- Double quotes (`"text"`) → always gives `&str`.
- Use `String` when text needs to change or be created at runtime.
- Use `&str` or `&String` when passing text around without owning it (efficient).
