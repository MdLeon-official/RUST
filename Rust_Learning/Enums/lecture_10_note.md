### Define Methods on Enums:

1. **Use the `impl` Block**: Implement methods for the enum using `impl EnumName`.
2. **Define a Method**: Inside the `impl`, define methods using the standard `fn` syntax.
   - Use `self` as the first parameter to refer to the enum instance.
   - Choose the appropriate ownership for `self`:
     - `self` for taking ownership.
     - `&self` for immutable borrowing.
     - `&mut self` for mutable borrowing.
3. **Match on `self`**: Use a `match` statement on `self` to define variant-specific behavior.
4. **Access Associated Data**: Handle references to associated data properly when borrowing.

---

### Example: Adding a `wash_laundry` Method to an Enum

```rust
// Define the enum
enum LaundryCycle {
    Cold, // No associated data
    Hot { temperature: u32 }, // Struct variant
    Delicate(String), // Tuple variant
}

// Implement methods for LaundryCycle
impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature.");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {}.", temperature);
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {}.", fabric_type);
            }
        }
    }
}

fn main() {
    // Create instances of the enum variants
    let cold_cycle = LaundryCycle::Cold;
    let hot_cycle = LaundryCycle::Hot { temperature: 100 };
    let delicate_cycle = LaundryCycle::Delicate(String::from("silk"));

    // Call the wash_laundry method on each instance
    cold_cycle.wash_laundry();
    hot_cycle.wash_laundry();
    delicate_cycle.wash_laundry();
}
```

---

### Key Points:
1. **Method Invocation**: Methods are called using dot notation, e.g., `instance.method_name()`.
2. **Ownership & Borrowing**: 
   - `&self` ensures the enum instance isn't consumed during the method call.
   - Associated data is accessed as references (e.g., `&u32`, `&String`) when using `&self`.
3. **Behavior per Variant**: Use `match` to define behavior for each variant and access associated data where necessary.

---

### Output of the Program:
```
Running the laundry with cold temperature.
Running the laundry with a temperature of 100.
Running the laundry with a delicate cycle for silk.
```
