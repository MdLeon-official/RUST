**Rust Enums Simplified**

An **enum** is a type that represents a set of possible values, called **variants**. The word enum comes from "enumerate," which means "to list one by one."

### Why Use Enums?
- Enums are useful when you need a type with limited, specific options.
- Example real-world enums:
  - Days of the week (7 variants: Monday, Tuesday, etc.)
  - Seasons of the year (4 variants: Spring, Summer, Fall, Winter)
  - Playing card suits (4 variants: Hearts, Diamonds, Spades, Clubs)

Using a string to represent these can lead to errors because strings can contain any text. Enums ensure that only valid options are used.

---
### How to Declare an Enum
1. Use the `enum` keyword.
2. Name the enum in **PascalCase** (capitalize the first letter of each word).
3. List the variants inside curly braces, separated by commas.

**Example:**
```rust
// Declare an enum for card suits
#[derive(Debug)] // Allows us to print the enum
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
```
---
### How to Use an Enum
1. Create a variable and set its type to the enum.
2. Use `::` to select a specific variant.

**Example:**
```rust
fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;

    // Change the variant (if the variable is mutable)
    second_card = CardSuit::Clubs;

    // Print the value (Debug must be derived)
    println!("{:?}", second_card); // Output: Clubs
}
```
---
### Enums in Other Structures
Enums can be used like any other type, such as:
- **Fields in structs:**
  ```rust
  struct Card {
      rank: String,
      suit: CardSuit,
  }
  ```
- **Elements in arrays:**
  ```rust
  let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
  ```
- **Values in tuples:**
  ```rust
  let card_tuple = (CardSuit::Hearts, CardSuit::Spades);
  ```
---
### Ownership with Enums
- Enums follow the same ownership rules as other types in Rust:
  - If an enum is assigned to a variable, the variable owns it.
  - If passed to a function, the function takes ownership.
  - If stored in a struct, array, or tuple, the container owns it.

---
### Key Takeaways
- Enums represent **a fixed set of options.**
- Variants are specific values within the enum.
- Enums ensure only valid values are used, making your program safer and less error-prone.
- Use `#[derive(Debug)]` to enable printing the enum with `println!("{:?}", variable)`.

**Example Recap:**
```rust
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

fn main() {
    let card = CardSuit::Hearts;
    println!("{:?}", card); // Output: Hearts
}
```

