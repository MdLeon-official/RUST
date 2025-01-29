# 🏗️ Module as file:


Rust provides **three ways** to define modules. Here, we focus on **moving a module to a separate file** for better organization.

---

## 🔴 **1st Approach (Inline Module in `main.rs`)**  
```rust
mod inventory {
    pub const MANAGER: &str = "Ivan Inventory";
}

fn main() {
    println!("Inventory Manager: {}", inventory::MANAGER);
}
```
✅ Works but makes `main.rs` longer as more modules are added.  

---

## 🟢 **2nd Approach (Module in a Separate File)**  
### 📂 **Project Structure**  
```
src/
 ├── main.rs        <-- Crate root (entry point)
 ├── inventory.rs   <-- Separate module file
```
### ✍ **Step 1: Create `inventory.rs`**  
Move module contents into **`src/inventory.rs`**:  
```rust
pub const MANAGER: &str = "Ivan Inventory";
```
✅ **No `mod inventory {}` block needed!**  
✅ The filename **becomes the module name automatically**.  

---

### ✍ **Step 2: Declare the Module in `main.rs`**  
In `src/main.rs`, just declare:  
```rust
mod inventory;  // Rust will automatically look for `inventory.rs`

fn main() {
    println!("Inventory Manager: {}", inventory::MANAGER);
}
```
✅ `mod inventory;` **tells Rust to look for `inventory.rs`**.  
✅ We **still access it with `inventory::MANAGER`**, just like before.  
✅ `main.rs` **stays clean and readable**.  

---

## 🛠️ **How Rust Resolves Module Locations**  
1️⃣ **Looks inside `{}` for inline definitions**.  
2️⃣ If `{}` is missing, **searches for `inventory.rs`** in `src/`.  
3️⃣ If that file is missing, **compilation fails**.  

---

## 🔑 **Key Takeaways**  
✅ Moving modules to **separate files** keeps `main.rs` clean.  
✅ Rust **automatically associates file names with module names**.  
✅ The **syntax for accessing items (`inventory::MANAGER`) stays the same**.  
✅ **Use `mod inventory;`** in `main.rs` to link the file.  

