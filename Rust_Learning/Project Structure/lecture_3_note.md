# 🌐 The pub keyword:

## 🔐 Privacy in Rust Modules  
By **default, all items inside a module are private**. This means they **cannot** be accessed from outside the module.  

## ❌ What Happens if We Try to Access a Private Item?  
```rust
mod inventory {
    const MANAGER: &str = "Ivan Inventory";
}

fn main() {
    println!("The manager of our inventory is {}", MANAGER);
}
```
### ❗ This will cause an error:  
```
error[E0425]: cannot find value `MANAGER` in this scope
```
### Why?  
- `MANAGER` is inside the `inventory` module.  
- The **main function cannot access private module items**.  

---

## 🛠️ Using the Scope Resolution Operator (`::`)  
To **access items inside a module**, we use the **scope resolution operator** (`::`):  
```rust
fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
}
```
⚠️ But this still **won't work** because `MANAGER` is **private**.  

---

## ✅ Making Items Public with `pub`  
To **allow access from outside the module**, we use the `pub` keyword:  
```rust
mod inventory {
    pub const MANAGER: &str = "Ivan Inventory";  // Now it's public!
}

fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
}
```
### ✅ Output:  
```
The manager of our inventory is Ivan Inventory
```

---

## 🔑 Key Takeaways  
✅ **By default, module items are private**.  
✅ **To make an item public, use `pub`** before it.  
✅ **Use `::` (scope resolution operator) to access public items** in a module.  
✅ **Every item must be explicitly marked `pub` if we want to access it outside the module**.  
