# Rust Enums with Associated Data

## Enum Variants with Associated Data
- Variants can store additional data using tuple-like syntax `(Type)`.
- Each variant can store different amounts or types of associated data.

### Declaring Enums with Unified Data
- Example where all variants store one piece of associated data:

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}
```

### Instantiating Enum Variants
- Use the variant name with a tuple of the respective data.
- Example of creating instances:

```rust
fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("5678"));
    let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298"));

    println!("{:?}", visa);
    println!("{:?}", mastercard);
}
```

### Key Points
- Associated data can be provided:
  - Inline during instantiation, e.g., `String::from("5678")`.
  - Using variables that hold the data.
- Rust enforces the correct data type and number of elements for each variant.

---

## Advanced: Variants with Multiple Data Fields
- A variant can store more than one piece of associated data:

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String, i32, bool), // Example with three fields
    DebitCard(String),
    PayPal(String),
}

fn main() {
    let card = PaymentMethodType::CreditCard(String::from("5678"), 123, true);
    println!("{:?}", card);
}
```

---

## Debug Representation
- Printing enums with `println!("{:?}", ...)` will show:
  - The variant name.
  - All associated data in the order it is declared.

Example Output:
```
CreditCard("5678")
DebitCard("2532-1298")
```

---

## Usage Tips
- Use enums with associated data when you need to:
  - Group related data under meaningful variants.
  - Ensure type safety for representing distinct states or categories.
- Examples:
  - Representing payment methods, states, or errors in an application.
