# The HashSet


### 📦 What is `HashSet`?

- `HashSet` is a collection **that only stores unique values**.
- It **prevents duplicates automatically**.
- Internally, it uses a `HashMap`.

### 🛠️ Why use it?

Use `HashSet` when:
- You need to **store items without repetition**.
- You want **fast checking, inserting, and removing** of values.

### 🔄 Real-life example:
> Online ticket queues prevent the same person from entering the line more than once.

---

### 🚀 Basic Usage

```rust
use std::collections::HashSet;

let mut concert_queue: HashSet<&str> = HashSet::new(); // explicitly specify type (&str)
concert_queue.insert("Molly");
concert_queue.insert("Megan");
```

🎯 Trying to add "Molly" again?
```rust
concert_queue.insert("Molly"); // silently ignored (no error)
```

🧠 **Memory tip**:  
> "Set" your data once — no twins allowed!

---

### 📋 Important Methods

#### ✅ `insert()`
Adds an item if it’s not already in the set.
```rust
concert_queue.insert("Fred");
```

#### ❌ `remove() -> bool`
Removes item if it exists. Returns `true` if removed.
```rust
concert_queue.remove("Megan"); // true
concert_queue.remove("Franny"); // false
```

#### 🔎 `contains() -> bool`
Checks if item is in the set.
```rust
concert_queue.contains("Molly"); // true
concert_queue.contains("Joe");   // false
```

#### 🎁 `get() -> Option<&T>`
Returns `Some(&item)` if it exists, or `None` otherwise.
```rust
concert_queue.get("Molly"); // Some("Molly")
concert_queue.get("Joe");   // None
```

📌 `get()` returns a reference to avoid ownership issues.

---

### 📏 Other Utilities

- `len()` — Returns number of elements.
```rust
concert_queue.len(); // e.g., 2
```

---

### 🔚 Summary

| Type        | Purpose             |
|-------------|---------------------|
| Array/Tuple | Maintain order      |
| HashMap     | Store key-value     |
| **HashSet** | Avoid duplicates ✅ |

HashSet = Unique Items + Fast Insert + Fast Check
