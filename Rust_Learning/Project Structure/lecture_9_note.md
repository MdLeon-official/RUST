# Submodules

### 1. **What Are Submodules?**
- Submodules are modules **nested** within other modules.
- Help organize code into **logical units**.
- Example: `products` is a submodule inside `inventory`.

### 2. **Declaring a Submodule**
There are three ways to declare a submodule:

#### **(1) Inline Module (Inside `inventory.rs`)**
```rust
mod products {
    pub struct Item {
        pub name: String,
    }
}
```

#### **(2) Separate File (`inventory/products.rs`)**
- In `inventory.rs`, declare:
```rust
pub mod products;
```
- Then, create `inventory/products.rs`:
```rust
pub struct Item {
    pub name: String,
}
```

#### **(3) Folder-Based (`inventory/products/mod.rs`)**
- In `inventory.rs`, declare:
```rust
pub mod products;
```
- Inside `inventory/products/`, create `mod.rs`:
```rust
pub struct Item {
    pub name: String,
}
```

### 3. **Accessing Submodules**
- In `main.rs`, access `Item` using:
```rust
use crate::inventory::products::Item;
```

### 4. **Visibility Rules**
- Modules and items are **private** by default.
- Use `pub` to expose them:
```rust
pub mod products {
    pub struct Item {
        pub name: String,
    }
}
```
- Even if `Item` is `pub`, its fields must also be `pub` for external access.

### 5. **Why Use Submodules?**
✅ Keeps code **organized**
✅ Helps with **code reuse**
✅ Makes large projects **manageable**

