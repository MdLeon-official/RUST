### The let else Construct

---

### Key Differences Between `if let` and `let else`
1. **`if let`:**
   - Executes a block if a dynamic value matches a hard-coded enum variant.
   - Variables declared are available **only within the block**.

2. **`let else`:**
   - Executes an `else` block if a dynamic value **does not match** a hard-coded enum variant.
   - Variables declared are available **after the `else` block** for the entire scope of the function.

---

### Syntax and Example
```rust
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    let Milk::Lowfat(percent) = my_beverage else {
        println!("You do not have low fat milk.");
        return; // Ensures `percent` isn't used if `my_beverage` is not `Lowfat`.
    };

    println!("{percent}% milk is available here.");
}
```

### Key Points:
1. **`else` block execution:** Runs only when the value doesn't match the pattern (e.g., `my_beverage` is not `Lowfat`).
2. **Variable availability:** Variables (like `percent`) are declared **only when the value matches** and can be used **after the `else` block.**
3. **Termination requirement in `else`:** The compiler requires the `else` block to terminate (e.g., with `return`) to avoid attempting to access undeclared variables.

---

### Expanded Example with Struct Variants
```rust
let my_beverage = Milk::NonDairy { kind: String::from("Almond") };

let Milk::NonDairy { kind } = my_beverage else {
    println!("You do not have nondairy milk.");
    return;
};

println!("{kind} milk is available here.");
```

---

### Use Cases
- Ensures variables are safely declared when matching specific patterns.
- Provides a clean way to handle mismatched patterns with custom error handling or fallback logic.

---

### General Flow
- **Match:** The variable(s) are declared and can be used in subsequent code.
- **Mismatch:** The `else` block runs, and the function typically terminates to prevent further invalid access.
