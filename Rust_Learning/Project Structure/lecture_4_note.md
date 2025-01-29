# 🏗️ The benefit of namespaces: 

One major advantage of **modules** is that they provide **namespaces**, allowing **duplicate names** to exist in different parts of the program.  

---

## ❌ The Problem: Name Collisions  
In a **single scope**, duplicate names are **not allowed**:  
```rust
const MANAGER: &str = "Ivan Inventory";
const MANAGER: &str = "Oliver Orderson";  // ❌ Error: Duplicate name!
```
### 🚨 Compiler Error:
```
error[E0428]: the name `MANAGER` is defined multiple times
```
Rust **does not allow duplicate names** in the **same scope**.

---

## ✅ The Solution: Using Modules  
Each module acts like a **separate folder**, preventing name collisions:  
```rust
mod inventory {
    pub const MANAGER: &str = "Ivan Inventory";
}

mod orders {
    pub const MANAGER: &str = "Oliver Orderson";
}

fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);
}
```
### ✅ Output:  
```
The manager of our inventory is Ivan Inventory
The manager of our orders is Oliver Orderson
```
### 🔥 Why does this work?  
- **Each `MANAGER` constant lives inside its own module (inventory & orders).**  
- **Rust does not see them as duplicates** because they exist in **different namespaces**.  

---

## 🔑 Key Takeaways  
✅ **Modules allow duplicate names** by **scoping them into different namespaces**.  
✅ **Use `::` (scope resolution operator) to access the correct module’s item**.  
✅ **Think of modules like folders on a hard drive**—you can have the same file name in different folders!  

