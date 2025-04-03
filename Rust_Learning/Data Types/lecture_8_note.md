# Rust: Type Casting with `as`

#### What is Casting?
- **Casting** is the process of converting a value from one type to another.
- In Rust, we use the **`as`** keyword to perform explicit type conversions.

---

### Casting Integer Types
Example:
```rust
fn main() {
    let miles_away = 50; // Inferred as i32
    let miles_away_i8 = miles_away as i8; // Cast to i8
    let miles_away_u8 = miles_away as u8; // Cast to u8
    
    println!("Original: {}", miles_away);       // Output: 50
    println!("As i8: {}", miles_away_i8);       // Output: 50
    println!("As u8: {}", miles_away_u8);       // Output: 50
}
```
- The **original variable remains unchanged**.
- The cast value must fit within the new type’s range.

---

### Casting Floating-Point Types
Example:
```rust
fn main() {
    let miles_away = 100.329032; // f64 by default
    let miles_away_f32 = miles_away as f32; // Convert to f32
    
    println!("Original (f64): {}", miles_away);
    println!("As f32: {}", miles_away_f32);
}
```
- **Floating-point numbers** can be cast between `f64` and `f32`.

---

### Casting Float to Integer (Truncation)
Example:
```rust
fn main() {
    let miles_away = 100.329032; // f64
    let miles_away_int = miles_away as i32; // Convert to i32 (truncates decimals)
    
    println!("Original: {}", miles_away);      // Output: 100.329032
    println!("As i32: {}", miles_away_int);    // Output: 100
}
```
- When **casting a float to an integer**, Rust **truncates** the decimal part (it **does not round**).
  - `100.329032 as i32` → `100`
  - `3.9 as i32` → `3`

---

### Summary:
- Use `as` for **explicit type conversion**.
- **Ensure the value fits** within the target type’s constraints.
- **Floats to integers truncate decimals**, they **do not round**.
- Original variables **remain unchanged**—casting creates a **new** value.
