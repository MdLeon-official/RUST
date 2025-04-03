**Boolean Data Type in Rust**

### What is a Boolean?
A **Boolean** (named after mathematician George Boole) represents **truth values** in programming. It can have only **two possible values**:
- `true`
- `false`

Think of Booleans like:
- Yes or No
- On or Off
- Present or Absent

### Boolean in Rust
- The Boolean type in Rust is **`bool`** (lowercase).
- A Boolean takes **1 byte (8 bits)** in memory, even though **1 bit** is enough for `true/false`. This is done for **performance reasons** in modern computers.

### Example:
```rust
let is_handsome: bool = true;
let is_silly: bool = false;
println!("Handsome: {}", is_handsome);
println!("Silly: {}", is_silly);
```
**Output:**
```
Handsome: true
Silly: false
```

### Boolean from Comparisons
Booleans are often the result of comparisons using **comparison operators**:
```rust
let age: i32 = 21;
let is_young = age < 35;  // true
```
- `>` (greater than)
- `<` (less than)
- `>=` (greater than or equal to)
- `<=` (less than or equal to)
- `==` (equal to)
- `!=` (not equal to)

### Boolean Methods
Rust has built-in methods that return `bool`:
```rust
let age: i32 = -40;
println!("Is positive? {}", age.is_positive()); // false
println!("Is negative? {}", age.is_negative()); // true
```

### Remembering Tip:
**Bool is like a light switch – it’s either ON (`true`) or OFF (`false`).**

