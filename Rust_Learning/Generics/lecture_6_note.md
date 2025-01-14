
### Generics in `impl` Blocks II

1. **Specific Type for `T`:**
   - Methods can be defined for structs with a specific type for the generic parameter `T`.
   - For example:
     ```rust
     impl TreasureChest<String> {
         fn clean_treasure(&mut self) {
             self.treasure = self.treasure.trim().to_string();
         }
     }
     ```
     This method only applies to `TreasureChest<String>`.

2. **Generic Type for `T`:**
   - To define methods for structs with any type for `T`, use a generic declaration in the `impl` block:
     ```rust
     impl<T> TreasureChest<T> {
         fn capital_captain(&self) -> String {
             self.captain.to_uppercase()
         }
     }
     ```
   - The `<T>` tells Rust that `T` is a generic type, enabling these methods to work on all variations of `TreasureChest<T>`.

### Examples and Use Cases
- The `capital_captain` method works for all `TreasureChest<T>` structs, regardless of the type of `T`.
- Methods like `clean_treasure`, which rely on `T` being a specific type (e.g., `String`), are only valid for `TreasureChest<String>`.

### Key Insights
- **Concrete vs. Generic Types:**
  - Rust treats types like `String` or `[&str; 3]` as concrete types.
  - Declaring `<T>` in the `impl` block tells Rust that `T` is a placeholder for any type, avoiding ambiguity with similarly named concrete types.
  
- **Scalability:**
  - Methods tied to a specific type (`String`) are limited to that type.
  - Generic methods (`<T>`) provide wider applicability, making them ideal for abstract or shared functionality.

