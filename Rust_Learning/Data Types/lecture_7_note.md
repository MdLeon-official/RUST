# Formatting Floats with Format Specifier


#### What are Format Specifiers?
- Format specifiers allow you to **customize how dynamic values** are represented when printed.
- You use the **colon (`:`)** symbol followed by a specifier inside curly braces `{}` within the `println!` macro.

#### Customizing Precision:
- You can control the **precision** (number of digits after the decimal point) by adding `.n` after the colon, where `n` is the number of digits.
  
Example:
```rust
let pi = 3.141592653589793;
println!("Pi with 2 digits: {:.2}", pi);  // Prints: 3.14
println!("Pi with 4 digits: {:.4}", pi);  // Prints: 3.1416
```
- In this case, **.2** means two digits after the decimal point, and **.4** means four digits.

**Tip**: The value of `pi` doesn't change; only its representation is altered for printing.

#### Two Ways to Use Format Specifiers:
1. **Inline format specifiers within the curly braces:**
   - Example:
   ```rust
   println!("Pi: {:.3}", pi);  // Directly inside the curly braces
   ```
   
2. **Using format specifiers as supplementary arguments:**
   - Example:
   ```rust
   println!("Pi: {0:.3}", pi);  // pi is passed separately, {0} matches the first argument
   ```
   - The `0` refers to the first argument passed to `println!`.

#### Key Takeaways:
- Format specifiers control **how** values are printed (e.g., precision).
- Use the **colon (`:`)** followed by `.n` to specify precision.
- You can pass dynamic values either directly inside the curly braces or as separate arguments in `println!`.
