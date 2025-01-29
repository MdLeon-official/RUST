# Module ambiguity:

### **Key Takeaways**
- **Only one definition per module**: You can define a module using **one** of the three methods—inline, as a file, or as a folder (`mod.rs`).
- **Rust throws an error if duplicate definitions exist**:
  - Example: Having both `orders.rs` and `orders/mod.rs` causes an ambiguity error.
  - Example: Having an inline module `mod orders {}` **and** a file-based or folder-based module with the same name leads to conflicts.
- **Fix:** **Keep only one module definition** by choosing the best structure for your project.

Would you like a **quick comparison** of when to use each module approach?
