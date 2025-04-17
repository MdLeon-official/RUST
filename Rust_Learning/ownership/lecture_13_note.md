# The Copy Trait with References


### 🧊 Example: `"Cookies and Cream"`

```rust
let ice_cream = "Cookies and Cream";
let dessert = ice_cream;
```

### 🔑 Core Idea:
- In Rust, **references** (like `&str`) **implement the `Copy` trait**.
- This means when you **assign a reference to another variable**, Rust **copies the reference**, **not the data**.

### 📦 Stack vs Heap:
- Stack data (like references) is **cheap to copy** because it's just an **address**, not the full data.
- `&str` is a **reference to string data**, and it could point to data in the **binary or heap**.

### 🧾 Ownership:
- Assigning a reference **doesn't move ownership**.
- Both `ice_cream` and `dessert` point to the **same data**.
- ⚠️ No duplication of the actual string, just the address.

### 🎯 Remember:
> Think of references like **two papers with the same house address**. Both point to the same house ("Cookies and Cream").
