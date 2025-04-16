# The push_str Method on a String Type


### 🆚 String Literals vs `String` Type
- **String Literal** (e.g. `"Boris"`):
  - Immutable
  - Stored in the **binary executable**
  - Treated as a **string slice** (`&str`)
  - **Cannot** be changed

- **`String` Type** (e.g. `String::from("Boris")`):
  - Heap-allocated
  - **Mutable**
  - Can **grow**, **shrink**, or **change**

---

### 🔧 How to Mutate a String
```rust
let mut name = String::from("Boris");
name.push_str(" Pask");
```
- `mut` keyword allows us to change the variable
- `push_str()` is used to **append** a string slice (`&str`) to a `String`

---

### 🧱 Behind the Scenes (Memory: Heap + Stack)

When you create a `String`, two areas are used:

#### 📦 Stack (Holds metadata)
- ✅ **Pointer**: Address to the string data on the heap
- ✅ **Length**: How many **bytes** the string currently uses
- ✅ **Capacity**: How many **bytes** are available (allocated space)

#### 📂 Heap (Holds actual string content)
- Text like `"Boris"` is stored here
- Heap gives **flexible space**, so it can grow or shrink

---

### 🧮 Example
```rust
let mut name = String::from("Boris"); // 5 bytes, maybe 10 capacity
name.push_str(" Pask");              // adds more bytes
```

#### Two scenarios:
1. ✅ **Enough Capacity**
   - Text is added in the same memory block
   - Length increases, capacity stays the same

2. ❌ **Not Enough Capacity**
   - Rust moves text to a new heap location with more space
   - Old heap memory is **deallocated**
   - Stack pointer now points to the **new location**

---

### 🌟 Why Use `String`?
- It’s **mutable**
- Can grow/shrink at runtime
- Useful when content **isn’t fixed**
