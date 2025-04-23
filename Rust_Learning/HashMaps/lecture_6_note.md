# The entry Method


#### 🔄 Problem with `.insert()`:
- It **overwrites** the existing value if the key exists.

---

### ✅ Solution: `.entry().or_insert()`

#### `entry(key)`
- Returns an enum: `Entry`
- Two variants:
  - `Occupied` → key **exists**
  - `Vacant` → key **does not exist**

#### `.or_insert(value)`
- Adds the value **only if** key is **not** present
- Prevents accidental overwrites
- Returns a **mutable reference** to the value (whether existing or newly inserted)

---

### 🧪 Example: Conditional Insert

```rust
use std::collections::HashMap;

let mut coffee_pairings = HashMap::new();
coffee_pairings.insert("Latte", "Oat Milk");

// Try to insert "Latte" again
coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
// Latte already exists → value stays "Oat Milk"

// Insert new key "Cappuccino"
coffee_pairings.entry("Cappuccino").or_insert("Pistachio Milk");
// Cappuccino doesn't exist → new pair added

println!("{:?}", coffee_pairings);
// Output:
// {"Latte": "Oat Milk", "Cappuccino": "Pistachio Milk"}
```

---

### 📝 Bonus Tip

You can also **modify** the value using the returned mutable reference:

```rust
coffee_pairings.entry("Latte").or_insert("Almond Milk").make_ascii_uppercase();
```

---

### 🔑 Summary

| Method             | Behavior                                   |
|--------------------|--------------------------------------------|
| `.insert()`        | Adds or **overwrites**                    |
| `.entry().or_insert()` | Adds **only if key is missing**       |
| Return Value       | `&mut V` (mutable reference to value)     |
