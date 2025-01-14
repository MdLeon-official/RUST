
### Generics and impl Blocks I:

1. **Struct with Generics**:
   - Structs can include a generic type `T` to provide flexibility in field types.
   - Example:
     ```rust
     struct TreasureChest<T> {
         captain: String,
         treasure: T,
     }
     ```

2. **Defining Methods with Specific Generics**:
   - Methods can be defined on structs for specific concrete types of `T` using `impl` blocks.
   - Syntax:
     ```rust
     impl TreasureChest<String> {
         fn clean_treasure(&mut self) {
             self.treasure = self.treasure.trim().to_string();
         }
     }
     ```
   - Here, `clean_treasure` only works on `TreasureChest` instances where `T` is `String`.

3. **Granularity in Method Definitions**:
   - You can define methods for very specific types, such as arrays with fixed sizes.
     ```rust
     impl TreasureChest<[&str; 3]> {
         fn amount_of_treasure(&self) -> usize {
             self.treasure.len()
         }
     }
     ```
   - This method only applies to `TreasureChest` structs where `T` is a 3-element array of string slices.

4. **Method Applicability**:
   - Methods defined in an `impl` block are only available for the specified concrete type of `T`.
   - Example:
     ```rust
     let mut silver_chest = TreasureChest {
         captain: String::from("Bloodsail"),
         treasure: String::from("    Silver    "),
     };

     silver_chest.clean_treasure(); // Works because T = String
     ```

5. **Demonstration**:
   - Example:
     ```rust
     let special_chest = TreasureChest {
         captain: String::from("Bootyplunder"),
         treasure: ["Gold", "Silver", "Platinum"],
     };

     println!("Amount of treasure: {}", special_chest.amount_of_treasure());
     ```

6. **Key Takeaway**:
   - Methods are tied to the specific type of `T`. Rust ensures type safety by disallowing method calls on incompatible types.
