### The Match keyword V

---

### Matching Enum Variants with Associated Data
Rust allows you to match not only on enum variants but also on specific associated data values. This is powerful for writing precise and expressive patterns.

---

### Example: Enum with Variants and Associated Data

Define an enum `Milk` with two variants:
- `Lowfat` (has an associated `i32` value for fat percentage).
- `Whole` (no associated data).

```rust
#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {}% version.", percent);
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

fn main() {
    let milk1 = Milk::Lowfat(1);
    milk1.drink(); // Output: You've got the lowfat 1% version.

    let milk2 = Milk::Lowfat(2);
    milk2.drink(); // Output: Delicious, 2% milk is my favorite!

    let milk3 = Milk::Whole;
    milk3.drink(); // Output: You've got the whole milk!
}
```

---

### Key Points:
1. **Matching Specific Values**:
   - Use the value directly in the pattern. For example, `Milk::Lowfat(2)` matches only the `Lowfat` variant with a `2`.

2. **General Matches**:
   - Use a variable to capture the associated data. For example, `Milk::Lowfat(percent)` matches any `Lowfat` variant and binds the `i32` value to `percent`.

3. **Order Matters**:
   - Specific patterns must come **before** general patterns. For example:
     ```rust
     Milk::Lowfat(2) => { ... } // Specific
     Milk::Lowfat(percent) => { ... } // General
     ```
     If reversed, the specific pattern for `2` becomes unreachable.

4. **Exhaustive Matching**:
   - Rust ensures all cases are covered. If any variant is unhandled, the compiler raises an error.

---

### Why is This Useful?
- Enables highly granular control over logic based on specific values.
- Reduces boilerplate by combining matching and data extraction.
