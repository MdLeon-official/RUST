# The crate Prefix

### Module Hierarchy
- Rust modules follow a hierarchical or tree-like structure.
- A parent module can contain child modules, resembling a folder structure.
- To reference a module, we use **paths**, which specify the module's location.

### Absolute Path
- Starts from the **crate root** (`main.rs` for a binary crate).
- Uses the `crate` keyword to explicitly begin from the top.
- Example:
  ```rust
  crate::inventory::MANAGER
  ```
- **Usage**: Useful when referring to items from the top level, ensuring clarity in large projects.

### Relative Path
- Starts from the **current module** instead of the crate root.
- Omits `crate`, assuming the reference starts within the current module.
- Example:
  ```rust
  inventory::products::ProductCategory::Ladder
  ```
- **Usage**: More convenient for referencing items within the same module or submodules.

### Best Practices
- List submodules at the **top** of a file for better organization.
- Use **relative paths** within the same module and **absolute paths** when clarity is needed.
- The `crate` keyword is optional in `main.rs` but useful in nested modules for clarity.


