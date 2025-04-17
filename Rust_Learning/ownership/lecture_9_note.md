# The clone Method



#### ✅ Stack data (like `i32`) is **Copy**
```rust
let x = 10;
let y = x; // Both x and y are valid.
```

#### ⚠️ Heap data (like `String`) is **Move by default**
```rust
let person = String::from("Boris");
let genius = person; // 'person' is now invalid.
```

---

### 🧪 To Avoid Moving Heap Data: Use `.clone()`
```rust
let person = String::from("Boris");
let genius = person.clone(); // clone makes a new copy on the heap
println!("{}", person); // ✅ Still valid!
```

### 📌 What Happened Here:
- `.clone()` made a **separate copy** of the heap data.
- Now **two variables** own their own separate heap strings.
- `Clone` is a trait → Types like `String` implement it.

---

### ⚠️ Important Notes:
- Cloning **uses more memory** (makes a full copy).
- Avoid `.clone()` **unless needed**.
- But... don’t stress too much when starting out — clone if it helps!
- Rust will guide you as you grow — you’ll learn to optimize later.
