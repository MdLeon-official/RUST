# Struct Update Syntax


When struct field names match variable or parameter names, Rust allows shorthand initialization:

```rust
struct Coffee {
    name: String,
    price: f32,
    is_hot: bool,
}

fn new_coffee(name: String, price: f32, is_hot: bool) -> Coffee {
    Coffee {
        name,      // shorthand for `name: name`
        price,     // shorthand for `price: price`
        is_hot,    // shorthand for `is_hot: is_hot`
    }
}
```

Works the same for variables:

```rust
let name = String::from("Latte");
let price = 3.99;
let is_hot = true;

let latte = Coffee { name, price, is_hot };
```

---

### 🔄 Struct Update Syntax

You can copy fields from one struct into another using `..other_struct`:

```rust
let mocha = Coffee {
    name: String::from("Mocha"),
    price: 3.49,
    is_hot: true,
};

// Only override the `name`, copy others from `mocha`
let caramel = Coffee {
    name: String::from("Caramel Macchiato"),
    ..mocha
};
```

#### 📌 Rules:
- `..other` must come **last** in the struct.
- Any field you define explicitly (like `name` above) won't be copied from the source.
- Fields implementing `Copy` (like `f32`, `bool`) are duplicated safely.
- `String` and other heap types **do not** implement `Copy`. Using `..mocha` will **move ownership**.

---

### ⚠ Ownership Warning

If all fields are copied using `..mocha`, ownership of non-`Copy` fields like `String` is **moved**:

```rust
let caramel = Coffee {
    ..mocha  // name (String) is moved here!
};

println!("{}", mocha.name); // ❌ Error! Value was moved
```

✅ To avoid move, clone the field:

```rust
let caramel = Coffee {
    name: mocha.name.clone(),  // Clone String to keep mocha usable
    ..mocha
};
```
