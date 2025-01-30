# The Use Keyword I:


### Purpose
- The `use` keyword brings names into the current scope, allowing us to avoid specifying full paths repeatedly.
- Acts as a shortcut for accessing names in nested modules.

### Syntax
```rust
use crate::module::submodule::Item;
```
- The last item in the path (`Item`) is imported and can be used directly.
- Does **not** change the underlying module structure, only affects imports in the current file.

### Example
Before `use`:
```rust
mod inventory;
fn main() {
    println!("{}", inventory::MANAGER);
}
```
After `use`:
```rust
mod inventory;
use crate::inventory::MANAGER;
fn main() {
    println!("{}", MANAGER);
}
```

### Importing Multiple Items
Use curly braces `{}` to import multiple names from the same module.
```rust
use crate::inventory::{MANAGER, FLOOR_SPACE, talk_to_manager};
```
- This allows direct access without needing `inventory::` prefix.
- Items are typically sorted alphabetically.

### Nested Module Example
Before `use`:
```rust
fn main() {
    let category = inventory::products::ProductCategory::Ladder;
}
```
After `use`:
```rust
use crate::inventory::products::ProductCategory;
fn main() {
    let category = ProductCategory::Ladder;
}
```

### Summary
- The `use` keyword simplifies code by reducing redundant module prefixes.
- Can be used to import single or multiple names.
- Helps in organizing and maintaining cleaner code.

