# Module as folder:

1. **Inline Module** (Single File, Using `{}`):
   ```rust
   mod inventory {
       pub const MANAGER: &str = "Alice";
   }
   ```
   - Everything is declared within curly braces in `main.rs`.

2. **Separate File** (`mod_name.rs` in `src/`):
   - In `main.rs`:
     ```rust
     mod inventory;
     ```
   - In `src/inventory.rs`:
     ```rust
     pub const MANAGER: &str = "Alice";
     ```
   - Rust looks for `inventory.rs` in `src/`.

3. **Separate Directory** (`mod.rs` inside `mod_name/`):
   - In `main.rs`:
     ```rust
     mod orders;
     ```
   - In `src/orders/mod.rs`:
     ```rust
     pub const STATUS: &str = "Pending";
     ```
   - Rust looks for a directory `orders/` and `mod.rs` inside it.

Using directories (`mod.rs`) is helpful for **large modules** with multiple submodules. A future improvement (Rust 2018+ edition) allows using `orders.rs` as a "mod entry file" instead of `orders/mod.rs`, making `mod.rs` unnecessary.

Would you like an example of organizing submodules within a directory?
