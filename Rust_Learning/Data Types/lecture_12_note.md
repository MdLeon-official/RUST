
# Boolean Inversion in Rust
- The **exclamation point (`!`)** is used to invert a Boolean value.
  - `!true → false`
  - `!false → true`

#### Example:
```rust
fn main() {
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;

    println!("Can I see this scary movie? {}", can_see_rated_r_movie);
    println!("Can I not see this scary movie? {}", cannot_see_rated_r_movie);
}
```
#### Explanation:
- `can_see_rated_r_movie` is `true` if `age >= 17`, else `false`.
- `cannot_see_rated_r_movie` is the **inverse** using `!`.




# Equality and Inequality Operators

1. **Equality operator (`==`)**:
   - Checks if two values are equal.
   - Returns `true` if they are equal, otherwise `false`.
   - Example: `"Coke" == "Coke"` will return `true`.

2. **Inequality operator (`!=`)**:
   - Checks if two values are **not** equal.
   - Returns `true` if they are not equal, otherwise `false`.
   - Example: `"Coke" != "Pepsi"` will return `true` because the strings are different.

### Case Sensitivity:
- String comparisons in Rust are **case-sensitive**.
  - `"Coke"` is not equal to `"coke"` because the first letter's case differs.
  
### Space Consideration:
- **Spaces** are valid characters, so `"Coke "` (with an extra space) is not equal to `"Coke"`.

### Comparing Different Data Types:
- Rust requires **matching data types** for comparisons:
  - An `i32` (integer) cannot be directly compared with a `f64` (floating-point number).
  - You can use the `as` keyword to convert data types, e.g., `13.0 as i32` to compare `13.0` (float) with `13` (integer).

### Comparison of Booleans:
- `true == true` returns `true`.
- `true != false` returns `true`.

### Practical Use Cases:
- **Equality** and **inequality** operators are especially useful in dynamic programs, such as user authentication (comparing user input to a stored value).

### Example Code:
```rust
fn main() {
    let str1 = "Coke";
    let str2 = "Pepsi";
    println!("{}", str1 == str2);  // false
    println!("{}", str1 != str2);  // true

    let num1 = 13;
    let num2 = 13.0;  // float
    println!("{}", num1 == num2 as i32);  // true after conversion

    let bool1 = true;
    let bool2 = false;
    println!("{}", bool1 == bool2);  // false
    println!("{}", bool1 != bool2);  // true
}
```

