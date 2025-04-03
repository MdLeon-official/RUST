### Rust: Mathematical Operators

Rust supports all the common mathematical operations using **operators**, which act on **operands** (values).

---

### 1️⃣ **Addition (`+`)**
```rust
fn main() {
    let addition = 5 + 4; // 9
    println!("Addition: {}", addition);
}
```
- `+` is the **addition operator**.
- `5` and `4` are the **operands**.

---

### 2️⃣ **Subtraction (`-`)**
```rust
fn main() {
    let subtraction = 10 - 6; // 4
    println!("Subtraction: {}", subtraction);
}
```
- `-` is the **subtraction operator**.

---

### 3️⃣ **Multiplication (`*`)**
```rust
fn main() {
    let multiplication = 3 * 4; // 12
    println!("Multiplication: {}", multiplication);
}
```
- `*` is the **multiplication operator**.

---

### 4️⃣ **Division (`/`)**
#### **Integer Division (Floor Division)**
```rust
fn main() {
    let floor_division = 5 / 3; // 1
    println!("Floor Division: {}", floor_division);
}
```
- `5 / 3` results in `1`, not `1.6667` because Rust **truncates** the decimal part.

#### **Floating-Point Division**
```rust
fn main() {
    let decimal_division = 5.0 / 3.0; // 1.6666667
    println!("Decimal Division: {}", decimal_division);
}
```
- **If one operand is a float (`5.0`), Rust returns a float result**.

---

### 5️⃣ **Modulo (`%`)**
```rust
fn main() {
    let remainder = 7 % 2; // 1
    println!("Remainder: {}", remainder);
}
```
- **Modulo (`%`) returns the remainder** of a division.
- `7 % 2 = 1` because `7 / 2` gives `3` with `1` left over.

| Expression  | Remainder |
|------------|----------|
| `7 % 2`   | `1`      |
| `8 % 2`   | `0`      |
| `9 % 2`   | `1`      |

---

### 📌 **Summary**
| Operator | Name             | Example     | Result |
|----------|----------------|-------------|--------|
| `+`      | Addition        | `5 + 4`     | `9`    |
| `-`      | Subtraction     | `10 - 6`    | `4`    |
| `*`      | Multiplication  | `3 * 4`     | `12`   |
| `/`      | Division        | `5 / 3`     | `1` (integer) |
| `/`      | Float Division  | `5.0 / 3.0` | `1.6667` |
| `%`      | Modulo          | `7 % 2`     | `1`    |

Rust **preserves integer types** in integer division and **truncates decimals** unless explicitly using floating points.
