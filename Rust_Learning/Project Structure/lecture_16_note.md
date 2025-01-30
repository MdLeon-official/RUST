# Using pub use to Export Names from Submodules:


In Rust, the `use` keyword is commonly used to import names from modules, and it can be used not only in `main.rs` but also inside any module. This allows us to bring names from submodules into the current file's scope.

#### Basic Import with `use`
For example, let's say we have an `inventory` module, and inside it, there's a `products` submodule containing an `Item` struct and a `ProductCategory` enum. Without `use`, you would have to refer to them with their full path like `inventory::products::Item`. But with `use`, you can bring them into the current scope for easier access:

```rust
use products::{Item, ProductCategory};
```

Now, you can use `Item` and `ProductCategory` directly without specifying the full path.

#### Re-exporting with `pub use`
You can also use `pub use` to **re-export** names from submodules, making them available at the parent module level. This means you can access `Item` and `ProductCategory` directly from `inventory` without worrying about the nested `products` submodule:

```rust
pub use products::{Item, ProductCategory};
```

With this, any code that imports `inventory` will automatically have access to `Item` and `ProductCategory`, without needing to know they're inside `products`.

#### Example:
1. **Before**: In the parent module (`inventory`), you'd need to access `Item` and `ProductCategory` like this:

   ```rust
   use inventory::products::{Item, ProductCategory};
   ```

2. **After `pub use`**: You can now simply do this:

   ```rust
   use inventory::{Item, ProductCategory};
   ```

#### Benefits of `pub use`:
- It **simplifies imports** by re-exporting names from submodules.
- The **code structure** remains clean, and the crate root doesn't need to know about the internal submodule structure.
- This allows **better organization** and encapsulation of the code while providing a simpler interface for external code.

#### Key Points:
- **`use`** imports names into the current module.
- **`pub use`** re-exports names from submodules to make them available at the parent module level.
- It helps in **simplifying imports** and **encapsulating** the internal structure of a module.

