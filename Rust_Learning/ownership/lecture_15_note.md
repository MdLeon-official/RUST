# Mutable Parameters


1. **Ownership Transfer**: 
   - When passing data to a function, ownership can be **moved** or **copied** depending on whether the type implements the `Copy` trait.
   - If a variable (like a `String`) doesn't implement the `Copy` trait, ownership will be transferred (moved) to the function parameter. The original variable can no longer be used after this transfer.

2. **Immutability of Function Parameters**: 
   - By default, function parameters are **immutable**, meaning you can't modify the data they hold unless you explicitly mark them as `mut`.
   - When transferring ownership to an immutable parameter, you can no longer mutate the data, as seen with the string "Burger" being passed to `add_fries`.

3. **Making Parameters Mutable**:
   - If you need to mutate the data in the function, you must declare the parameter as `mut`.
   - This allows you to use methods like `push_str` to modify the string inside the function.

4. **Rust Compiler Warnings**:
   - The compiler will warn you if you're using a mutable variable but not modifying it. It helps keep your code safe and clean by suggesting unnecessary `mut` keywords be removed.

### Key Takeaways:
- **Ownership transfer** means you lose access to the original variable after passing it to a function.
- **Immutability** is the default for function parameters. To mutate data inside the function, use `mut` with the parameter.
- The **Rust compiler** helps by providing warnings when you misuse mutability or ownership, making your code more robust and easier to maintain.
