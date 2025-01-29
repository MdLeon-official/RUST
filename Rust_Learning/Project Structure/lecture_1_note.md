# 📦 Packages & Crates  

## 🚀 Overview  
Rust projects can grow beyond a single `main.rs` file. To manage larger projects, we use **packages** and **crates**.  

## 📌 Packages & Crates  
- **Package** = A folder with a `Cargo.toml` file (created using `cargo new`).
- **Crate** = The smallest unit of code that Rust compiles. A package must contain **at least one** crate.

## 🏗 Types of Crates  
1. **Binary Crate** 🏃  
   - Compiles into an **executable program**.  
   - Has a `main.rs` file with a `main()` function.  
   - Runs independently.  

2. **Library Crate** 📚  
   - Does **not** have a `main()` function.  
   - Provides reusable functionality for other crates.  
   - Commonly used as external dependencies.  

## 📂 Project Structure  
When you create a Rust project (`cargo new warehouse`), it generates:  
```
warehouse/
│── Cargo.toml  # Package metadata
└── src/
    ├── main.rs  # Binary crate (if exists)
    ├── lib.rs   # Library crate (if exists)
```
- If `main.rs` exists → **Binary crate** ✅  
- If `lib.rs` exists → **Library crate** ✅  
- Both can exist together in a package.  

## 🔑 Key Takeaways  
- Every Rust project (package) has **at least one crate** (binary or library).  
- **Binary crates** are executable programs, while **library crates** provide reusable code.  
- Cargo detects `main.rs` (binary) and `lib.rs` (library) automatically.  

