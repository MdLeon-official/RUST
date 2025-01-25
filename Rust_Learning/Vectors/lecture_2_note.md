## Adding and removing elements: 

### **Mutability**
- To modify a vector (add or remove elements), it must be declared as `mut`:
  ```rust
  let mut pizza_diameters = vec![8, 10, 12];
  ```

### **Adding Elements**
1. **`push` Method**:  
   - Appends a value to the end of the vector.
   - Example:
     ```rust
     pizza_diameters.push(16); // Adds 16 to the end
     ```
2. **`insert` Method**:  
   - Inserts a value at a specific index, pushing subsequent elements down by one.
   - Example:
     ```rust
     pizza_diameters.insert(0, 4); // Adds 4 at index 0
     ```

### **Removing Elements**
1. **`pop` Method**:  
   - Removes the last element and returns it as an `Option` (`Some` if successful, `None` if the vector is empty).
   - Example:
     ```rust
     let last_pizza_diameter = pizza_diameters.pop(); // Removes and returns the last value
     ```
   - Handles empty vectors gracefully:
     ```rust
     if let Some(value) = pizza_diameters.pop() {
         println!("Removed: {}", value);
     } else {
         println!("Vector is empty!");
     }
     ```

2. **`remove` Method**:  
   - Removes an element by index and returns it.
   - If the index is invalid, it will panic at runtime.
   - Example:
     ```rust
     let third_diameter = pizza_diameters.remove(2); // Removes element at index 2
     ```
   - **Runtime Error Example**:
     ```rust
     pizza_diameters.remove(50); // Panics if index > vector length
     ```

### **Key Differences Between Methods**
- **`push`**: Adds to the end of the vector.
- **`insert`**: Adds at a specific index and shifts elements.
- **`pop`**: Removes and returns the last element (safe with `Option`).
- **`remove`**: Removes an element at a specific index (panics on invalid index).

---

### **Advantages of Vectors Over Arrays**
- Vectors can dynamically expand or shrink during runtime, making them more versatile than fixed-size arrays.
