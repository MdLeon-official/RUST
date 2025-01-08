### The Match keyword III

1. **Enum with Associated Data**:
   - Enums can have variants with different types of associated data:
     - **Unit Variant**: No associated data.
     - **Struct Variant**: Named fields like a struct.
     - **Tuple Variant**: Ordered fields like a tuple.

2. **Example Enum Declaration**:
   ```rust
   enum LaundryCycle {
       Cold,                      // Unit variant
       Hot { temperature: u32 },  // Struct variant
       Delicate(String),          // Tuple variant
   }
   ```

3. **Match Syntax for Enum Variants**:
   - **Unit Variant**: No associated data; use the variant name directly.
   - **Struct Variant**: Access fields using curly braces `{}`.
   - **Tuple Variant**: Access values by order using parentheses `()`.

4. **Example Function**:
   ```rust
   fn wash_laundry(cycle: LaundryCycle) {
       match cycle {
           LaundryCycle::Cold => {
               println!("Running the laundry with cold temperature");
           },
           LaundryCycle::Hot { temperature } => {
               println!("Running the laundry with a temperature of {temperature}");
           },
           LaundryCycle::Delicate(fabric_type) => {
               println!("Running the laundry with a delicate cycle for {fabric_type}");
           },
       }
   }
   ```

5. **Main Function**:
   ```rust
   fn main() {
       // Using the Cold variant
       wash_laundry(LaundryCycle::Cold);

       // Using the Hot variant with a temperature field
       wash_laundry(LaundryCycle::Hot { temperature: 100 });

       // Using the Delicate variant with a String value
       wash_laundry(LaundryCycle::Delicate(String::from("silk")));
   }
   ```

6. **Output**:
   ```
   Running the laundry with cold temperature
   Running the laundry with a temperature of 100
   Running the laundry with a delicate cycle for silk
   ```

---

### Detailed Syntax for Match Arms

1. **Unit Variant**:
   - No associated data; match directly.
   ```rust
   LaundryCycle::Cold => { ... }
   ```

2. **Struct Variant**:
   - Use field names inside curly braces `{}` to destructure and access data.
   ```rust
   LaundryCycle::Hot { temperature } => { ... }
   ```

3. **Tuple Variant**:
   - Use parentheses `()` and positional names to destructure and access data.
   ```rust
   LaundryCycle::Delicate(fabric_type) => { ... }
   ```

---

### Key Insights

- Struct and tuple variant syntax mirrors how data is declared in the enum:
  - **Struct Variant** uses curly braces for named fields.
  - **Tuple Variant** uses parentheses for ordered fields.
- Rust ensures all variants are matched, enhancing safety and avoiding runtime errors.
- The `match` statement not only handles all possibilities but also allows easy access to associated data.
