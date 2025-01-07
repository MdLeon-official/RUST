# Enum Variants in Rust

## Types of Enum Variants
Rust offers **three kinds of enum variants**, which mirror the types of structs:

### 1. **Unit-Like Variant**
- No associated data.
- Equivalent to a **unit-like struct**.
- Example:  
  ```rust
  enum PaymentMethod {
      Cash,
  }
  ```

### 2. **Tuple Variant**
- Stores data in sequential order.
- Equivalent to a **tuple-like struct**.
- Example:  
  ```rust
  enum PaymentMethod {
      CreditCard(String),
  }
  ```

### 3. **Struct Variant**
- Stores named fields with types.
- Equivalent to a **named-field struct**.
- Example:  
  ```rust
  enum PaymentMethod {
      PayPal { username: String, password: String },
  }
  ```

---

## Benefits of Struct Variants
1. **Named Fields**: Provide clarity and context for associated data.
   - Example: `PayPal { username: String, password: String }` is more explicit than `(String, String)`.

2. **Less Code**: Eliminates the need for a separate struct declaration when tied directly to an enum variant.

---

## Comparing Tuple Variants and Struct Variants
| **Aspect**            | **Tuple Variant**                             | **Struct Variant**                           |
|------------------------|-----------------------------------------------|----------------------------------------------|
| **Clarity**            | Relies on order of data.                     | Provides named fields for better readability.|
| **Code Complexity**    | Requires parentheses for instantiation.      | Uses curly braces for named fields.          |
| **Flexibility**        | Easier to declare inline.                    | Allows field-level context and validation.   |

---

### Syntax Comparison
#### Tuple Variant:
```rust
enum PaymentMethod {
    CreditCard(String),
}

let method = PaymentMethod::CreditCard(String::from("0012-3456"));
```

#### Struct Variant:
```rust
enum PaymentMethod {
    PayPal { username: String, password: String },
}

let method = PaymentMethod::PayPal {
    username: String::from("bob@gmail.com"),
    password: String::from("password"),
};
```

---

### Advantage of Using External Struct
- Decouples the type from the enum, allowing **reuse** elsewhere in the code.
- Example:
  ```rust
  struct Credentials {
      username: String,
      password: String,
  }

  enum PaymentMethod {
      PayPal(Credentials),
  }
  ```

---

## Summary
- Rust enums are versatile, supporting variants that store no data, sequential data, or named data.
- Use the appropriate variant type depending on your clarity and reuse needs:
  - Use **unit-like** variants for simple, constant options.
  - Use **tuple variants** for simple, sequential data.
  - Use **struct variants** for clear, named, and context-rich data.
