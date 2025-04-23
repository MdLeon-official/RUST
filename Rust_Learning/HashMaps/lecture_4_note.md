# Access a Value by Key


#### ✅ Method 1: **Square Brackets**
```rust
let value = coffee_pairings["Flat White"];
```
- Directly retrieves the value.
- **Panics at runtime** if key doesn’t exist.
- Unsafe for unknown/missing keys.

---

#### ✅ Method 2: **`.get()` Method**
```rust
let value = coffee_pairings.get("Flat White");
```
- Returns an `Option<&V>`:
  - `Some(&value)` if key exists
  - `None` if key doesn't exist
- **Doesn’t panic**, safer than brackets.

---

### 🌀 Why `.get()` Returns a Reference
- Avoids **moving ownership** out of the `HashMap`
- Ensures the map **retains full ownership**

---

### 📦 Extracting Actual Value from Option
```rust
let value = coffee_pairings.get("Flat White").copied();
```
- `.copied()` converts `Option<&T>` → `Option<T>` (if `T: Copy`)
- Works well with `&str` since it's `Copy`

---

### 🔓 Handling `None` Safely
```rust
let value = coffee_pairings.get("Cappuccino").copied().unwrap_or("Unknown Milk");
```
- `.unwrap_or()` lets you provide a fallback
- Avoids runtime panic
- Ensures you always get a usable value

---

### 🚨 Summary
| Method         | Safe? | Ownership | Panics if Key Missing? | Returns         |
|----------------|-------|-----------|-------------------------|-----------------|
| `map[key]`     | ❌ No | Takes value if owned | ✅ Yes               | Value           |
| `map.get(key)` | ✅ Yes | Keeps ownership       | ❌ No                | `Option<&V>`    |
