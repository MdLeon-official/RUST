
## Generics in Structs



1. **Generic Declaration**:  
   - After the struct name, declare generics within angle brackets `<T>`.  
   - These generics act as placeholders for types, determined when an instance of the struct is created.

   ```rust
   struct TreasureChest<T> {
       captain: String,  // Always a String.
       treasure: T,      // Flexible type determined at runtime.
   }
   ```

2. **Flexible Field Types**:  
   - The `captain` field is a concrete `String`.
   - The `treasure` field uses the generic `T`, allowing different types like:
     - Scalar types (e.g., `&str`, `String`).
     - Compound types (e.g., arrays, tuples, structs).

---

### Creating Struct Instances
- **Example with a String Slice**:
  ```rust
  let gold_chest = TreasureChest {
      captain: String::from("Firebeard"),
      treasure: "Gold", // T is inferred as &str.
  };
  ```
  
- **Example with a Heap-Allocated String**:
  ```rust
  let silver_chest = TreasureChest {
      captain: String::from("Bloodsail"),
      treasure: String::from("Silver"), // T is inferred as String.
  };
  ```

- **Example with a Collection**:
  ```rust
  let special_chest = TreasureChest {
      captain: String::from("Bootyplunder"),
      treasure: ["Gold", "Silver", "Platinum"], // T is inferred as an array of &str.
  };
  ```

---

### Printing Structs with Debug
- Deriving the `Debug` trait allows you to print struct instances.
  ```rust
  #[derive(Debug)]
  struct TreasureChest<T> {
      captain: String,
      treasure: T,
  }

  fn main() {
      let gold_chest = TreasureChest {
          captain: String::from("Firebeard"),
          treasure: "Gold",
      };

      let silver_chest = TreasureChest {
          captain: String::from("Bloodsail"),
          treasure: String::from("Silver"),
      };

      let special_chest = TreasureChest {
          captain: String::from("Bootyplunder"),
          treasure: ["Gold", "Silver", "Platinum"],
      };

      println!("{:?}", gold_chest);
      println!("{:?}", silver_chest);
      println!("{:?}", special_chest);
  }
  ```

---

### Benefits of Generics in Structs
- **Reusability**: One struct can support various types without redefining it multiple times.
- **Flexibility**: You can use any type for the generic field, including:
  - Built-in types (`i32`, `&str`, `String`).
  - User-defined types (structs, enums).
  - Collections (arrays, tuples).
- **Type Safety**: The compiler enforces type correctness, ensuring mismatched types are caught during compilation.

---

### Summary
Generics simplify code design, improve flexibility, and eliminate redundancy, enabling a single struct like `TreasureChest` to handle multiple use cases. Whether dealing with scalar values, compound data, or custom types, generics empower Rust developers to write cleaner, reusable code.
