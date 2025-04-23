# Hash Maps and Ownership


### ✅ Basic Behavior
- A `HashMap` **owns** its keys and values.
- If you insert a value that doesn’t implement `Copy`, ownership **moves** to the map.
- After inserting, original variables become **invalid**.

```rust
let mut coffee_pairings = HashMap::new();

let drink = String::from("Latte");
let milk = String::from("Oat Milk");

coffee_pairings.insert(drink, milk); // drink and milk are now moved
// drink and milk can't be used here
```

---

### 🛠️ Keeping Variables Valid: Two Options

#### 1. Clone
Creates **new heap allocations**, keeping the originals valid.

```rust
coffee_pairings.insert(drink.clone(), milk.clone());
```

#### 2. Use References
Pass `&String` to insert without taking ownership.

```rust
coffee_pairings.insert(&drink, &milk);
// drink and milk still valid
```

---

### 📌 Type Inference Caveat
- Using `&String` means HashMap type is inferred as `HashMap<&String, &String>`.
- You **can’t insert** `&str` (`"text"`) directly in this case.

#### Solution: Use string slices as generic type:
```rust
let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
coffee_pairings.insert(&drink, &milk); // works
coffee_pairings.insert("Flat White", "Almond Milk"); // also works
```

- Thanks to **deref coercion**, `&String` can become `&str`.

---

### 🧮 HashMap Length
```rust
coffee_pairings.len(); // returns number of key-value pairs
```

---

### 📚 Summary
- Rust enforces ownership in HashMaps.
- Use `.clone()` to duplicate or references (`&String`) to borrow.
- Prefer `&str` as key/value types if you need flexibility.
- Understand deref coercion for mixed insertions.

