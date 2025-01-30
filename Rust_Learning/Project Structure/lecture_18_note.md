# The Standard Library:



The **Rust Standard Library** is a collection of built-in modules that provide essential functionality for most Rust programs. It includes constructs such as:

- **Structs**
- **Enums**
- **Functions**
- **Constants**
- **Types**
- **Nested modules** for organizing functionality

Rust aims for consistency, and the syntax for importing modules or names from the standard library is the same as when using our own modules or external crates.

#### Key Points:
1. **Top-level module**: The root of the standard library is `std` (short for "standard").
2. **Accessing modules**: To access functionality from the standard library, use the `use` keyword followed by the module name, such as `std::fmt` for formatting or `std::io` for input/output.
3. **Modules and Submodules**: The standard library is organized into modules, which can have their own submodules. For example:
   - `std::fmt` for formatting functions.
   - `std::io` for input/output operations, like reading from stdin or writing to stdout.

#### Importing from the Standard Library:
You can import entire modules or specific items (e.g., functions) from them:

```rust
use std::{fmt, io};
use std::io::{stdin, stdout};
```

- **Consolidating imports**: You can combine multiple imports from the same module on a single line using curly braces:
  ```rust
  use std::{fmt, io::{stdin, stdout}};
  ```

- **Using `self` keyword**: To refer to the current module, you can use `self`:
  ```rust
  use std::io::self;
  ```

#### The Purpose of the Standard Library:
The standard library provides many useful constructs that save time and effort, reducing the need to write these functionalities from scratch. It includes modules for file handling, networking, collections, and much more.

