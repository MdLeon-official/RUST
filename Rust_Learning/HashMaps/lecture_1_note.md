# Create a HashMap with new Function


#### 💡 What is a HashMap?
- A **collection** of **key-value pairs**.
- Each **key is unique**, but **values can be duplicated**.
- Also known as: `dictionary`, `map`, `hash`, `associative array` (in other languages).

#### 📌 Use a HashMap when:
- You want to **associate two values**.
- You **don’t care about order** (use `Vec` or `Array` for ordered data).
- You want to **search by key** to get the value quickly.

#### 🛠️ Syntax Basics:
```rust
use std::collections::HashMap;

let mut menu: HashMap<String, f64> = HashMap::new();

menu.insert(String::from("Steak"), 29.99);
menu.insert(String::from("Tuna"), 29.99);  // values can be duplicates
menu.insert(String::from("Burger"), 14.99);

// Debug print
println!("{:?}", menu);
```

#### 🌍 Real World Example:
- Menu item (`"Steak"`) = key
- Price (`29.99`) = value

---

### 🏗️ Creating HashMaps

#### ✅ Explicit with Types:
```rust
let mut menu: HashMap<String, f64> = HashMap::new();
```

#### ✅ With `&str` keys/values:
```rust
let mut country_capitals: HashMap<&str, &str> = HashMap::new();
country_capitals.insert("France", "Paris");
country_capitals.insert("Germany", "Berlin");
```

#### ✅ Using Turbofish Operator:
```rust
let mut country_capitals = HashMap::<&str, &str>::new(); // ::<K, V>::new()
```

---

### 🧠 Key Concepts

- **Keys must be unique** – they act like search indexes.
- **Values can repeat** – they’re the result we want.
- Stored on the **heap** – like `Vec` and `String`, because size is not known at compile time.
- Supports **mutation**: add, remove, update.

---

### 🛑 Common Errors

- Rust **cannot infer key/value types** if the map is empty and has no insertions.
- Fix: either add insertions or specify types manually.
