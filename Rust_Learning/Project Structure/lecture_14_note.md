# The super Keyword:


## 1. Creating a Constructor for a Struct  
In Rust, constructor functions for structs are just regular functions that return an instance of the struct. By convention, the constructor function is named `new()`.  

### Example:  
```rust
impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        Self { name, category, quantity }
    }
}
```
🔹 The function is **public** (`pub`) so it can be accessed outside the module.  
🔹 `Self` is used instead of writing `Item`.  
🔹 Uses shorthand syntax (`name, category, quantity`) to initialize fields.  

---

## 2. Modules and Parent-Child Relationship  
Modules in Rust follow a **parent-child** structure:  
- A **submodule** (child) is inside another module (parent).  
- To **navigate down**, write the module name (`inventory::products`).  
- To **navigate up**, use the `super` keyword.  

---

## 3. Accessing the Parent Module with `super`  
A **submodule can access its parent module** using `super`.  

### Example:  
```rust
impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();  // Calls function from parent module
        Self { name, category, quantity }
    }
}
```
🔹 `super::talk_to_manager()` accesses a function in the **parent module** (`inventory`).  
🔹 The child (`products` submodule) **automatically** has access to the parent’s private functions!  

---

## 4. Privacy in Modules  
🚀 **A child module can access a private function in its parent!**  
Even if `talk_to_manager()` is private (`fn talk_to_manager()`), the `products` submodule can still use it.  

🔹 **Public (`pub`) only matters for external code** trying to access a module.  
🔹 **A child module is always allowed to access its parent’s functions, even private ones.**  

---

### Summary:  
✅ `new()` constructor function initializes a struct.  
✅ Use `super` to **access the parent module**.  
✅ Child modules **can access private functions** in their parent.  
