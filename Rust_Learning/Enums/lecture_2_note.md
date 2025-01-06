# Rust Enums with Associated Data

## Enum Basics
- An **enum** is a type that represents a set of possible values.
- Each possible value in an enum is called a **variant**.
- Enums can also store additional data (associated values) with their variants.

### Declaring an Enum
1. Use the `enum` keyword.
2. Write the name of the enum in **PascalCase**.
3. Use curly braces `{}` to list the variants.
4. Derive the `Debug` trait for easy printing using `#[derive(Debug)]`.

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard,
    DebitCard,
    PayPal,
}
```

### Creating Enum Instances
- Use the `enum` name and the `::` syntax to reference a specific variant.

```rust
fn main() {
    let visa = PaymentMethodType::CreditCard;
    println!("{:?}", visa);
}
```

## Enum Variants with Associated Data
- Variants can store data, similar to struct fields.
- Use parentheses `()` after the variant name to define associated data types.

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}
```

### Example with Associated Data
```rust
fn main() {
    // Creating instances with associated data
    let visa = PaymentMethodType::CreditCard(String::from("5678"));
    let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298"));

    // Printing instances
    println!("{:?}", visa);
    println!("{:?}", mastercard);
}
```

### How It Works
- The data types declared with each variant must match the data provided during instantiation.
- For example:
  - `CreditCard(String)` expects a `String`.
  - Use `String::from("...")` to create a `String`.

## Real-World Use Cases
Enums with associated data are useful when:
- A variant needs extra information to fully represent its value.
- Examples:
  - Payment methods (e.g., `CreditCard` with a card number).
  - States in a game (e.g., `Playing(score: u32)`, `Paused`).
  - Error handling (e.g., `IoError(String)`, `ParseError`).

### Behind the Scenes
- Each variant becomes a function that:
  - Constructs an instance of the enum.
  - Accepts the associated data as arguments.
- Example:
  - `PaymentMethodType::CreditCard(String::from("5678"))` calls the constructor for `CreditCard`.

### Debug Representation
- Use `println!("{:?}", ...)` to print the enum.
- Includes the variant name and its associated data.

### Ownership Rules
- Enum instances follow Rust's ownership principles.
- Moving an enum to a new variable transfers ownership of its associated data.
- Passing an enum to a function gives the function ownership of the enum.

### Variants with Multiple Data Types
- Variants can store multiple pieces of data.

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String, i32, bool),
    DebitCard(String),
    PayPal(String),
}

fn main() {
    let card = PaymentMethodType::CreditCard(String::from("5678"), 123, true);
    println!("{:?}", card);
}
```

## Summary
- Enums represent a limited set of possible values.
- Variants can have associated data to store additional information.
- Use enums to create more structured and meaningful types in your Rust programs.


