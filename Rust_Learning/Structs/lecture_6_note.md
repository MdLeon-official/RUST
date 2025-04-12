# Struct Field Initialization Shorthand Syntax


When you define a struct and then create an instance of it, if your variable or parameter names **match** the field names of the struct, Rust lets you **skip writing `field_name: value`** and just write the name once.

---

### 🧱 Example Struct
```rust
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}
```

---

### 🔧 Normal way (explicit field assignment)
```rust
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
```

---

### 🚀 Shortcut (field init shorthand)
```rust
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,     // shorthand
        price,    // shorthand
        is_hot,   // shorthand
    }
}
```
- ✅ Works **only** when parameter/variable name == struct field name.

---

### 📦 Using variables instead of parameters
```rust
fn main() {
    let name = String::from("Latte");
    let price = 3.99;
    let is_hot = false;

    let latte = Coffee { name, price, is_hot };
    // Rust links them automatically by name!
}
```

---

### 🧠 Why use this shorthand?
1. **Cleaner and shorter** code.
2. Avoids repetitive `field_name: field_name`.
3. Encourages consistency in naming.
