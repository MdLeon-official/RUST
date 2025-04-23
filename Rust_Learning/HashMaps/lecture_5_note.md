# Overwriting a Value with an Existing Key


### 🗂 `HashMap` Keys Must Be Unique

- Keys **identify values** → must be unique
- If a key already exists and you insert it again, the **value gets replaced**

---

### 🔁 Behavior of `.insert()` Method

#### ✅ Inserting a new key-value pair:
```rust
coffee_pairings.insert("Mocha", "Hazelnut Milk");
```
- Adds the pair if the key **doesn’t exist**

#### 🔄 Inserting with an **existing key**:
```rust
coffee_pairings.insert("Latte", "Pistachio Milk");
```
- Overwrites the **previous value**
- `"Latte": "Oat Milk"` becomes `"Latte": "Pistachio Milk"`

---

### 🧪 Example (Before & After)

```rust
let mut coffee_pairings = HashMap::new();
coffee_pairings.insert("Latte", "Oat Milk");

println!("{:?}", coffee_pairings); // {"Latte": "Oat Milk"}

coffee_pairings.insert("Latte", "Pistachio Milk");

println!("{:?}", coffee_pairings); // {"Latte": "Pistachio Milk"}
```

---

### ⚠️ Reminder

- `.insert()` replaces silently — no error or warning.
- Be careful when modifying existing entries.

If you want to insert **only if the key doesn’t exist**, you can use:
```rust
coffee_pairings.entry("Latte").or_insert("Oat Milk");
```

