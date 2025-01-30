# The use Keyword II (Name Conflicts):


## `use` Keyword Overview
- The `use` keyword imports names into the current file's scope, allowing shorter references.
- It creates a shortcut to a nested module, reducing the need for full paths.

## Handling Name Conflicts
- Rust **does not** allow duplicate names in the same scope, leading to name conflicts.
- If two modules (e.g., `inventory` and `orders`) have the same constant (e.g., `MANAGER`), importing both using `use` causes ambiguity.
- The compiler will warn about such conflicts.

## Best Practices
- When a name is shared across modules, use the **full path** (e.g., `inventory::MANAGER`) instead of importing it with `use`.
- This maintains **clarity** and prevents confusion about which module the name belongs to.
- Rust allows a **mix-and-match** approach: some names can be fully qualified while others are imported using `use` for brevity.

## Example
```rust
mod inventory {
    pub const MANAGER: &str = "Inventory Manager";
    pub fn talk_to_manager() {
        println!("Talking to inventory manager");
    }
}

mod orders {
    pub const MANAGER: &str = "Orders Manager";
}

use inventory::{talk_to_manager}; // Import function only, avoiding name conflict

fn main() {
    println!("{}", inventory::MANAGER); // Full path for clarity
    println!("{}", orders::MANAGER);    // Full path for clarity
    talk_to_manager(); // Shortcut for function
}
```
