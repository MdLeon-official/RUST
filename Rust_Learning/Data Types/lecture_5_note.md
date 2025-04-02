# Rust Methods

A Rust program consists of one or many functions. So far, we've talked about the `main` function, which runs automatically when the program starts.

### Functions vs Methods
- A **function** is a procedure or recipe—a sequence of steps to follow in order.
- A **method** is a function that belongs to a specific value or type.
- Methods are invoked on values using **dot syntax**: `value.method_name()`.

### Example: Absolute Value Method (`abs`)
```rust
fn main() {
    let value: i32 = -15;
    println!("The absolute value is: {}", value.abs());
}
```
#### Explanation:
- `value.abs()` calls the `abs` method on `value`, returning its absolute value.
- The return value of `abs` is **15**, which is printed.

### Example: Removing Whitespace (`trim`)
```rust
fn main() {
    let empty_space = "   my content   ";
    println!("Trimmed string: '{}'", empty_space.trim());
}
```
#### Explanation:
- `trim()` removes leading and trailing spaces from the string.
- The return value is **"my content"**, with no spaces at the beginning or end.

### Example: Raising to a Power (`pow`)
```rust
fn main() {
    let value: i32 = -15;
    println!("-15 squared is: {}", value.pow(2));
    println!("-15 cubed is: {}", value.pow(3));
}
```
#### Explanation:
- `pow(n)` raises `value` to the power of `n`.
- `-15.pow(2) → 225`, `-15.pow(3) → -3375`.

### Key Takeaways:
1. **Methods** belong to specific types and are invoked using `value.method()`.
2. **Parentheses** are required for method invocation, even when no arguments are passed.
3. **Return values**: Methods produce an output, which can be stored or used directly.
4. **Arguments**: Some methods accept arguments, like `pow(2)`.
5. **Multiple arguments**: Separate them with commas (`method(arg1, arg2, ...)`).

### Summary
Methods provide built-in functionality for Rust's types. You'll commonly use methods for:
- **Numbers** (e.g., `abs()`, `pow()`)
- **Strings** (e.g., `trim()`, `to_uppercase()`, `len()`)
