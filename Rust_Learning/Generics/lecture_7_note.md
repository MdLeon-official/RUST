### **Generics with Enums**
1. **Defining a Generic Enum:**
   - Enums can have a generic type parameter `T`, allowing the type of associated data to vary.
   - Example:
     ```rust
     enum Cheesesteak<T> {
         Plain,
         Topping(T),
     }
     ```
   - The `T` in `Topping(T)` means the associated data's type can be specified when the enum is used.

2. **Creating Enum Instances:**
   - Rust can infer the type of `T` when the associated data is provided:
     ```rust
     let mushroom = Cheesesteak::Topping("mushroom");
     // `T` inferred as `&str` (string slice).

     let onions = Cheesesteak::Topping("onions".to_string());
     // `T` inferred as `String`.

     let bacon = Cheesesteak::Topping(&"bacon".to_string());
     // `T` inferred as `&String`.
     ```

3. **Using Variants Without Associated Data:**
   - When using variants without associated data (e.g., `Plain`), Rust cannot infer the type of `T`:
     ```rust
     let plain = Cheesesteak::Plain;
     // Error: Rust doesn't know what type `T` is.
     ```
   - To fix this, you must explicitly annotate the type of `T`:
     ```rust
     let plain: Cheesesteak<String> = Cheesesteak::Plain;
     ```

4. **Why Explicit Annotation is Necessary:**
   - Even though the `Plain` variant doesn't use `T`, Rust requires the type to be known to handle potential reassignment:
     ```rust
     let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
     plain = Cheesesteak::Topping("topping".to_string()); // Valid.
     plain = Cheesesteak::Topping("invalid"); // Error: Expected `String`, found `&str`.
     ```

---

### **Key Takeaways**
- **Inference:** Rust infers the type of `T` when it can deduce it from the associated data.
- **Explicit Declaration:** For variants without associated data, you must explicitly annotate `T` to prevent ambiguity.
- **Compiler Checks:** Rust's strict type system ensures consistency, even for reassignment of enum variants.
