### The Match keyword II

1. **Match Arms with Blocks**:
   - Instead of returning a direct value, a match arm can use a block of code (`{}`) to execute multiple lines of logic.
   - The last evaluated value in the block becomes the block's return value.
   - Ensure consistency: all match arms must produce values of the same type.

2. **Mixing Syntax**:
   - It's possible to mix direct values and blocks in match arms.
   - Example:
     ```rust
     match os {
         OperatingSystem::Windows => {
             println!("Quite an old operating system!");
             39 // Final evaluated value returned by the block
         },
         OperatingSystem::MacOS => 23,
         OperatingSystem::Linux => 34,
     }
     ```

3. **Implicit Returns**:
   - The last line in a block, without a semicolon, is treated as the block's return value.
   - Adding a semicolon would turn the line into a statement, producing a unit type (`()`), which would likely cause a type mismatch error.

4. **Example Code**:
   ```rust
   enum OperatingSystem {
       Windows,
       MacOS,
       Linux,
   }

   fn years_since_release(os: OperatingSystem) -> u32 {
       match os {
           OperatingSystem::Windows => {
               println!("Quite an old operating system!");
               39
           },
           OperatingSystem::MacOS => 23,
           OperatingSystem::Linux => 34,
       }
   }

   fn main() {
       let my_computer = OperatingSystem::Linux;
       let age = years_since_release(my_computer);
       println!("My computer's operating system is {age} years old");

       let dads_computer = OperatingSystem::Windows;
       let age = years_since_release(dads_computer);
       println!("My dad's computer is {age} years old");
   }
   ```

5. **Output**:
   ```
   My computer's operating system is 34 years old
   Quite an old operating system!
   My dad's computer is 39 years old
   ```

---

### Practical Insights

- Using blocks within match arms provides more flexibility, allowing additional logic or side effects, such as logging, before returning a value.
- Rust enforces type consistency across all match arms, maintaining safety and predictability.
- By using blocks, you can integrate complex logic while still leveraging the power of `match` for control flow.
