# Mutable Parameters


In Rust, function parameters are **immutable by default**, just like variables. This means you can't change their values inside the function unless you explicitly declare them as mutable.

#### Key Concepts:
1. **Immutability by Default**:
   - If you want to modify a parameter within a function, you must declare it as `mut` (mutable).
   - Without this, you can't perform mutative operations like `.push_str()` on the parameter.

2. **Ownership Transfer**:
   - When you pass an owned value (like a `String`) into a function, **ownership moves** from the calling function to the function's parameter.
   - **Important:** The **parameter is immutable by default**, so it can't modify the value unless it's declared as mutable.

#### Example:
```rust
fn add_fries(meal: String) {
    meal.push_str(" and fries"); // Error because meal is immutable
}
```

- Even if the original `burger` string is mutable, ownership is moved to `meal`, which is immutable. Thus, you cannot mutate it inside the function.

3. **Making Parameters Mutable**:
   - To mutate a parameter inside the function, you must declare it as `mut`.
   
```rust
fn add_fries(mut meal: String) {
    meal.push_str(" and fries"); // Works now because meal is mutable
}
```

4. **Why Mutable `burger` Isn't Enough**:
   - **Ownership** is what's important, not whether the original variable is mutable. When ownership is transferred, you need to ensure that the new owner (parameter) is mutable if you want to modify it.

5. **Compiler Warnings**:
   - Rust will warn you if a variable is mutable but isn't mutated. In the example, if `burger` isn't changed, the compiler might tell you it doesn't need to be mutable.

#### Remembering Tip:
- Think of **ownership** like a **ticket**. You transfer the ticket (ownership) to someone else, and that person can either keep it mutable or immutable. If they keep it immutable, they can't make changes, but if it's mutable, they can!
