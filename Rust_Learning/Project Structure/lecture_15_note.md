# Create Aliases with the as Keyword:

When working with modules in Rust, sometimes we may encounter conflicts between names, like two modules having the same name for a variable or constant. For example, both the `inventory` and `orders` modules might have a `MANAGER` constant. If we try to import both `MANAGER` names into the same file, Rust will get confused about which one to use.

To solve this, we can use **aliases**. An alias is like giving something a nickname. This helps us avoid name conflicts while keeping our code clean.

#### The `as` Keyword
The `as` keyword is used to create aliases. This works just like casting types, but here it’s used to give a new name to an import. For instance:

```rust
use inventory::MANAGER as INVENTORY_MANAGER;
use orders::MANAGER as ORDERS_MANAGER;
```

Now, instead of directly using `MANAGER`, we use the aliases `INVENTORY_MANAGER` and `ORDERS_MANAGER`. This solves the problem of name conflicts because the name `MANAGER` doesn't exist anymore in this file.

#### Simplifying Imports
You can also combine multiple imports from the same module into a single line using curly braces. For example:

```rust
use inventory::{MANAGER as INVENTORY_MANAGER, FLOOR_SPACE};
```

This is cleaner and easier to manage when dealing with multiple imports.

### Key Points:
- **Alias**: A nickname for an import to avoid name conflicts.
- Use the `as` keyword to create an alias.
- Aliases only exist within the current file and import.

