# Overwrite Struct Fields


- **Variables are immutable by default** in Rust.
- To change any field of a struct, **the entire struct must be declared as mutable** using `mut`.
- You **cannot make only one field mutable**—it’s all or nothing.
- The **struct definition itself doesn't determine mutability**. Only its instances do.
- When you overwrite a field (especially a heap-allocated value like a `String`), the **old value is automatically dropped** and **ownership transfers** to the new one.

---

### ✅ **Syntax to Overwrite Struct Fields**

```rust
struct Coffee {
    name: String,
    price: f32,
    is_hot: bool,
}

fn main() {
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,
    };

    // Overwrite field values
    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;

    // Print updated struct values
    println!(
        "My {} this morning cost ${}. It is {} that it was hot.",
        beverage.name, beverage.price, beverage.is_hot
    );
}
```

---

### 📌 **Takeaways**

- Use `let mut` to enable modification.
- Use `struct_instance.field_name = new_value` to overwrite.
- Memory safety is preserved—Rust automatically handles deallocation of the previous value (if needed).
- This mutation is only possible *if* the struct instance is mutable.
