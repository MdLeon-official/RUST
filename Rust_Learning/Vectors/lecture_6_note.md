## Writing vector elements:

### **Mutating Elements in a Vector**
1. **Make the Vector Mutable**: Use the `mut` keyword to declare a mutable vector if you plan to change it:
   ```rust
   let mut pizza_toppings = vec!["Pepperoni", "Mushroom", "Sausage"];
   ```

2. **Overwriting Elements**: Use the index to overwrite an element:
   ```rust
   pizza_toppings[1] = "Olives"; // Replaces "Mushroom" with "Olives".
   println!("{:?}", pizza_toppings); // Prints: ["Pepperoni", "Olives", "Sausage"]
   ```

---

### **Mutating an Element's Content**
- **Borrowing and Mutating**:
  - Borrow a **mutable reference** to the element to modify its content without taking ownership:
    ```rust
    let target_topping = &mut pizza_toppings[2]; // Mutable reference to "Sausage".
    target_topping.push_str(" and Meatballs");  // Modifies "Sausage" to "Sausage and Meatballs".
    ```
  - The changes reflect in the vector:
    ```rust
    println!("{:?}", pizza_toppings); // Prints: ["Pepperoni", "Olives", "Sausage and Meatballs"]
    ```

---

### **Ownership & References Rules Recap**
1. **No Multiple Mutable References**:
   - You cannot have more than one mutable reference at a time:
     ```rust
     let mut_ref1 = &mut pizza_toppings[2];
     let mut_ref2 = &mut pizza_toppings[2]; // ERROR: Already a mutable reference exists.
     ```

2. **Lifetimes of References**:
   - Rust tracks the lifetime of references.
   - Once a mutable reference's lifetime ends (e.g., after it is last used), a new reference (mutable or immutable) can be created safely:
     ```rust
     let target_topping = &mut pizza_toppings[2];
     target_topping.push_str(" and Meatballs"); // Mutable reference used here.

     let another_topping = &pizza_toppings[2]; // Immutable reference created after the mutable one ends.
     println!("{}", another_topping);
     ```

3. **Multiple Immutable References**:
   - You can create as many immutable references as you want, and they can coexist:
     ```rust
     let ref1 = &pizza_toppings[0];
     let ref2 = &pizza_toppings[1];
     println!("{} and {}", ref1, ref2);
     ```

---

### **Key Takeaways**
- Rust ensures safety by enforcing **ownership** and **borrowing rules** at compile time.
- **Mutable references** allow modifications but are exclusive.
- **Immutable references** are non-exclusive and can coexist.
- Understanding **lifetimes** helps you write safe and optimized code.

