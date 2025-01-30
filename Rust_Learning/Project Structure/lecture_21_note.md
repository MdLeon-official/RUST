# Multiple Binary Crates:


1. **Package Structure**: A Rust package must have at least one crate, which can be a binary crate or a library crate. You can also have multiple binary crates, and they can coexist with the library crate.

2. **Binary Crates**: Binary crates are stored in the `src/bin/` directory. Each `.rs` file in this directory is treated as an independent binary crate. These crates represent separate programs that can run independently but share the same library crate.

3. **Library Crate**: The library crate is stored in the `lib.rs` file, which holds reusable logic and code. This crate is intended to be used by other binary crates within the same project or even by external projects when published on `crates.io`.

4. **Creating Multiple Binary Crates**: You can create multiple files within the `bin/` folder. Each file represents an independent executable. These executables can coexist with or replace the `main.rs` file.

5. **Running Multiple Crates**: When there are multiple binary crates, the command `cargo run` will require additional specifications. You use the `--bin` flag to indicate which binary crate to run. For example:
   - `cargo run --bin warehouse` to run the binary crate from `main.rs`.
   - `cargo run --bin summary` to run the binary crate from `bin/summary.rs`.
   - `cargo run --bin create_product_category` to run the binary crate from `bin/create_product_category.rs`.

6. **Structure of Crates**: 
   - `lib.rs`: Stores reusable code for shared logic (e.g., structs, functions, enums).
   - `bin/`: Stores multiple executable crates (e.g., separate programs like `summary.rs`, `create_product_category.rs`).

This design allows you to structure your project with both reusable code and independent executable programs. Each binary crate can access the shared logic from the library crate.

