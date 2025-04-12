# Create Structs in a Function



#### ✅ Goal:
Create a `Coffee` struct instance inside a function (`make_coffee`) and return it.

---

1. **Move Struct to Top-Level:**
   ```rust
   struct Coffee {
       name: String,
       price: f64,
       is_hot: bool,
   }
   ```
   - Declared outside `main()` so it's accessible by any function in the file.

2. **Define a Function to Return the Struct:**
   ```rust
   fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
       Coffee {
           name: name,
           price: price,
           is_hot: is_hot,
       }
   }
   ```
   - Uses **implicit return** (no semicolon).
   - Parameters match the struct fields.

3. **Calling the Function in `main`:**
   ```rust
   fn main() {
       let name = String::from("Latte");
       let coffee = make_coffee(name, 4.99, true);

       println!(
           "My {} this morning cost ${}. It is {} that it was hot.",
           coffee.name, coffee.price, coffee.is_hot
       );
   }
   ```

---

### 🔁 Ownership Flow:
1. `name` variable owns the String `"Latte"`.
2. `make_coffee(name, ...)` — ownership of the String moves to the parameter `name`.
3. Inside `make_coffee`, the `name` parameter's ownership is passed to the `Coffee` struct's `name` field.
4. The `Coffee` instance owns all fields, and is returned to the `coffee` variable.
5. Now, `coffee` is the **final owner** of the struct, and therefore the `name` String as well.

---

### 🧠 Key Takeaways:
- Structs can be **return types** in functions.
- Ownership moves step-by-step as values are passed:
  - Variable → Function parameter → Struct field → Returned to caller.
- You **must return** the struct to retain ownership; otherwise, it is dropped at the end of the function scope.
- Rust will infer types, but you can add explicit type annotations if needed:  
  ```rust
  let coffee: Coffee = make_coffee(...);
  ```
