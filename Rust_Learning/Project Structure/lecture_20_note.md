# Create Library Crate:


1. **Binary Crate**: 
   - A binary crate is a Rust program that compiles into an executable (final program). 
   - It is defined by the `main.rs` file.
   
2. **Library Crate**:
   - A library crate contains reusable code meant to be shared and used by other programs or libraries.
   - It is defined by the `lib.rs` file within the `src` directory.

#### Key Concepts:

- **A Package**: 
   - A Rust project, which is a directory containing a `Cargo.toml` file. A package can contain one or both types of crates (binary and library).
   
- **Creating a Library Crate**:
   - To create a library crate, simply add a `lib.rs` file to the `src` directory.
   - This file will serve as the entry point for the library, where modules like `inventory` and `orders` can be declared.

- **Relationship Between `main.rs` and `lib.rs`**:
   - The `main.rs` file typically contains the logic for the binary crate.
   - The `lib.rs` file, on the other hand, is the root for the library crate. This file can export modules and functions for use in other programs.

- **Using a Library Crate**:
   - If a project contains both a binary crate (`main.rs`) and a library crate (`lib.rs`), the binary crate can reference the library crate.
   - For this, use the `use` keyword followed by the crate's name (as specified in `Cargo.toml`) to bring in the library's modules.

   Example:
   ```rust
   use warehouse::inventory;  // Importing from the 'warehouse' library crate
   ```

- **Making Items Public**:
   - By default, items in `lib.rs` are private. To make them accessible from outside the library, you need to mark them as `pub`.
   - This applies to modules and specific items within modules.
   
   Example:
   ```rust
   pub mod inventory; // Making the 'inventory' module public
   ```

- **Exporting Items from the Library Crate**:
   - You can control what is available to users of your library by marking specific items as `pub`.
   - Items at the top level of the library crate can also be marked as `pub`, making them directly accessible.

   Example:
   ```rust
   pub const FLOOR_SPACE: u32 = 100;
   pub struct Item { /* ... */ }
   ```

- **Publishing on `crates.io`**:
   - Once you have a library crate, you can upload it to `crates.io`, the official Rust package registry, making it available for other developers to use.
   - Other developers can then add your library as a dependency in their own projects.

#### Example Workflow:
1. Create `main.rs` (binary crate) and `lib.rs` (library crate) files.
2. Define your library crate's modules and make them public using `pub`.
3. In the `main.rs` file, import and use the library crate.
4. The library crate is reusable and can be shared by uploading it to `crates.io`.

#### Conclusion:
- A **binary crate** produces a program (executable), while a **library crate** produces reusable code that can be used by other programs.
- A package can contain both types of crates, and Rust distinguishes them based on the presence of `main.rs` (binary) and `lib.rs` (library).
- Properly managing visibility and accessibility of items in your library crate with `pub` is crucial for enabling the right level of access.
