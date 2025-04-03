# Augmented Assignment Operators in Rust

## Mutable Variables & Reassignment
- In Rust, to modify a variable's value, it must be **mutable** (`mut`).
- Example:
  ```rust
  let mut year = 2025;
  year = year + 1;  // Updates 'year' to 2026
  println!("The new year is {}", year);
  ```
- Rust evaluates the right-hand side first (`year + 1`) and then assigns the result back to `year`.

## Augmented Assignment Operators
These operators simplify mathematical operations and reassignment:

| Operator | Meaning | Example |
|----------|---------|---------|
| `+=` | Add & Assign | `year += 1;`  (Same as `year = year + 1`) |
| `-=` | Subtract & Assign | `year -= 5;`  (Same as `year = year - 5`) |
| `*=` | Multiply & Assign | `year *= 2;`  (Same as `year = year * 2`) |
| `/=` | Divide & Assign | `year /= 4;`  (Same as `year = year / 4`) |

### Example Usage:
```rust
let mut year = 2025;
year += 1;  // year = 2026
year -= 5;  // year = 2021
year *= 2;  // year = 4042
year /= 4;  // year = 1010
println!("Final year: {}", year);
```

## **Important Notes**
- Rust **does not** support `++` (increment) or `--` (decrement) like some other languages.
- Always **declare a variable as `mut`** if you want to update its value.

### **Memory Tip 🧠:**
Think **left to right:**
1. First, apply the operation (`+`, `-`, `*`, `/`).
2. Then, assign the result back using `=`.

Using `+=`, `-=`, `*=`, `/=` makes code cleaner and shorter!

