# The remove Method


### 🗂️ `HashMap::from` - Create a HashMap from an array of tuples
- You can create a HashMap **with initial values** using `HashMap::from`.
- The input is an **array of tuples**, where each tuple is a `(key, value)` pair.

```rust
use std::collections::HashMap;

let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];
let years_at_company = HashMap::from(data);
```

- Tip: `from` = shortcut for filling up a HashMap fast.

---

### 🗑️ `.remove(key)` - Remove a key-value pair
- Removes a key-value pair **by key**.
- Returns an `Option<V>`:
  - `Some(value)` if key existed and was removed.
  - `None` if key didn’t exist.

```rust
let mut years = HashMap::from([("Ben", 6)]);
let ben = years.remove("Ben"); // returns Some(6)
let ben_again = years.remove("Ben"); // returns None
```

- To get the actual value:
```rust
println!("{:?}", ben.unwrap()); // prints 6
```
> ⚠️ If you `unwrap()` a `None`, it will **panic** (crash at runtime).

---

### 🔁 `Option` Recap:
```rust
enum Option<T> {
    Some(T),  // means we got a value
    None,     // means no value found
}
```

- Use `match`, `if let`, or `unwrap()` to handle the result.

---

### 🧠 Easy Memory Trick:
- `from` → **Fills** the HashMap with data
- `remove` → **Removes** data using a key
- `Option` → Result of a **search**; could find something (`Some`) or not (`None`)

