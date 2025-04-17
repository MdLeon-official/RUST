# Return Values I


In Rust, when a function **returns an owned value**, the ownership of that value is transferred back to the calling function.

#### Key Concepts:
1. **Returning Ownership**:
   - When you return a value from a function (like a `String`), ownership is transferred from the function's variable to the calling function's variable.
   - The calling function **becomes the new owner** of that value.

2. **Example**:
```rust
fn bake_cake() -> String {
    let cake = String::from("Chocolate Mousse");
    return cake;
}
```
- In this example, ownership of `cake` is moved from the `bake_cake` function to the `main` function when it's returned.

3. **Simplifying the Code**:
   - You can simplify the return by omitting the `return` keyword and using an expression directly.
```rust
fn bake_cake() -> String {
    String::from("Chocolate Mousse")  // Implicit return
}
```
- This still follows the same ownership transfer, but it’s more concise.

4. **Deallocation**:
   - If the returned value isn't assigned to any variable (for example, if we just call `bake_cake()`), the value is **dropped automatically** and the memory is deallocated.

5. **Important Principle**:
   - The key principle here is that Rust ensures that **ownership is always tracked**. Even if you don't assign the returned value to a variable, Rust ensures it is deallocated correctly.

#### Remembering Tip:
- Think of **ownership transfer** like **passing a baton**. When the function ends, it **passes the baton** (ownership) to the calling function. If no one takes the baton, it gets dropped (deallocated).
