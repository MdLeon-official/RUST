# Rust Enums with Associated Data (ii)

## Different Data for Different Variants
- **Associated values** allow each variant of an enum to store a unique collection of data types.
- Variants can:
  - Store different amounts of data.
  - Store data of different types.

### Example: Payment Method with Varying Data
- `CreditCard` and `DebitCard` store one `String` for an account number.
- `PayPal` stores two `String` values: an email address and a password.

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),           // Stores an account number
    DebitCard(String),            // Stores an account number
    PayPal(String, String),       // Stores email and password
}
```

### Instantiating Variants with Varying Data
- Example of creating instances for variants with different associated data:

```rust
fn main() {
    // Declare a mutable variable
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("5678"));

    // Reassign to PayPal variant
    my_payment_method = PaymentMethodType::PayPal(
        String::from("bob@gmail.com"), 
        String::from("password")
    );

    // Print the current state of the enum
    println!("{:?}", my_payment_method);
}
```

### Explanation of the Code
1. The variable `my_payment_method` is initially assigned to the `CreditCard` variant.
   - Requires one `String` value (account number).
2. The variable is later reassigned to the `PayPal` variant.
   - Requires two `String` values (email and password).
3. The `println!("{:?}", my_payment_method)` statement:
   - Prints the variant name (`PayPal`).
   - Prints the associated values (email and password).

---

## Key Points
- **Flexibility**: Variants can store different types and numbers of data.
- **Type Safety**: The Rust compiler enforces the correct data types and number of values for each variant.

### Debug Representation
- Example output for the `PayPal` variant:
  ```
  PayPal("bob@gmail.com", "password")
  ```

---

## Best Practices
- Use enums with associated values to:
  - Represent states or entities that have shared logic but distinct data.
  - Avoid mismatches between data types and their meanings.
