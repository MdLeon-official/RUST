# Rust Floating Point Types

#### What are Floating Points?
- A **floating-point number** represents decimal numbers (with fractions).
- Rust has two floating-point types:
  - **f32 (32-bit)**: Less precision, uses 4 bytes.
  - **f64 (64-bit)**: More precision, uses 8 bytes. Default in Rust.

#### Precision and Trade-offs:
- **f64** (64-bit) gives **15-17 digits** of precision.
- **f32** (32-bit) gives only **6-7 digits** of precision.
- **f64** is the default type for most cases, but use **f32** if memory is highly constrained.
  
**Tip**: When dealing with very large numbers or requiring high precision, prefer **f64**.

#### Example of Precision Loss:
- If you store **pi** with more than 15 digits, the number gets rounded.
- **f64** can handle numbers like `3.14159265358979`, but when you extend it, precision is lost.

```rust
let pi: f64 = 3.14159265358979323846;
println!("Pi value: {}", pi);  // Shows 15 digits max
```

**Tip**: If you need precision for calculations, use **f64**. If memory is a concern, opt for **f32**.

#### Methods on Floating Points:
Rust provides methods for rounding and manipulating floats:
1. **floor()**: Rounds down to the nearest whole number.
   - Example: `3.14.floor()` → **3**
2. **ceil()**: Rounds up to the nearest whole number.
   - Example: `3.14.ceil()` → **4**
3. **round()**: Rounds to the closest integer.
   - Example: `3.14.round()` → **3**, but `3.5.round()` → **4**

**Tip**: Think of the **floor** as "going down" and **ceil** as "going up."

#### Syntax:
- To use these methods, specify the type of the float explicitly (like **f32** or **f64**).
- Use a dot (`.`) followed by the method name (e.g., `floor()`, `ceil()`).

#### Key Takeaways:
- **f64** for most precision, **f32** for memory-saving when precision isn't as critical.
- Use methods like **floor**, **ceil**, and **round** to handle decimal rounding.
