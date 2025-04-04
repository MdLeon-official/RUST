# Explicit Return Values


#### 📌 Concepts:
- **Parameter**: Name for expected input (`number: i32`)
- **Argument**: Actual value passed during function call (`square(5)`)
- **Return value**: The **output** a function gives back to the **caller**

---

#### 🛠️ Syntax:

```rust
fn function_name(parameter: Type) -> ReturnType {
    return value;
}
```

- Use `-> Type` after parentheses to **declare return type**
- Use `return` to **return a value**
- **Semicolon required** after `return`

---

#### 🔁 Example:

```rust
fn square(number: i32) -> i32 {
    return number * number;
}

fn main() {
    let result = square(5);
    println!("The square of five is {result}"); // Output: 25
}
```

---

#### ⚠️ Notes:
- `return` **ends** the function. Code **after** it won’t run.
- Return type and parameter types can differ.
- If return type doesn't match declared type → **Compiler Error**
- You can **reuse** the function with different arguments.

---

#### ✅ Cleaner Rust Style (without `return`):

```rust
fn square(number: i32) -> i32 {
    number * number  // last expression is returned automatically
}
```
