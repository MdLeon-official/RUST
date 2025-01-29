# 🏗 Rust Modules:

## 📌 What is a Module?  
A **module** is a container that groups related code together. Think of it like a **folder** on your computer—it holds related files (constants, functions, structs, enums, etc.).  

## 📂 Creating a Module  
- Use the `mod` keyword followed by a **snake_case** name:  
  ```rust
  mod inventory {
      // Code inside this module
  }
  ```
- The module creates a **namespace**, preventing conflicts with other parts of the program.  

## 🎯 Example: Warehouse Inventory  
```rust
mod inventory {
    // Constants
    const FLOOR_SPACE: i32 = 10_000;
    const MANAGER: &str = "Ivan Inventory";

    // Enum for product categories
    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    // Struct for inventory items
    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    // Function to interact with the manager
    fn talk_to_manager() {
        println!("Hey {}, how's your coffee?", MANAGER);
    }
}
```
### 🔹 What’s Inside?
- **Constants:** `FLOOR_SPACE` and `MANAGER`  
- **Enum:** `ProductCategory` (Ladder, Hammer)  
- **Struct:** `Item` (name, category, quantity)  
- **Function:** `talk_to_manager()` (accesses `MANAGER`)  

## 🔑 Key Takeaways  
✅ A **module** organizes related code like a folder.  
✅ It creates a **namespace** to prevent naming conflicts.  
✅ Modules can contain **constants, functions, structs, and enums**.  
✅ Code inside a module is **not accessible by default**—we’ll see how to use it next! 🚀  
