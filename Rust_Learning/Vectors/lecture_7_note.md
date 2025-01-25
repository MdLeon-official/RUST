## Vector Capacity behind the scene: 

1. **Capacity**: A vector has a capacity, which is the maximum number of elements it can store without reallocating memory. When the vector's size exceeds its capacity, Rust allocates new space, moves the old elements to the new location, and deallocates the old memory.

2. **`with_capacity` Function**:
   - You can initialize a vector with a specific capacity using `with_capacity`:
     ```rust
     let mut seasons = Vec::with_capacity(4); // Creates a vector with capacity for 4 elements.
     ```

3. **Checking Capacity and Length**:
   - `len` gives the number of elements in the vector.
   - `capacity` gives the maximum number of elements the vector can hold without needing to reallocate:
     ```rust
     println!("Length: {}", seasons.len()); // Number of elements.
     println!("Capacity: {}", seasons.capacity()); // Maximum capacity.
     ```

4. **Adding Elements**:
   - Use the `push` method to add elements:
     ```rust
     seasons.push("Summer");
     seasons.push("Fall");
     seasons.push("Winter");
     seasons.push("Spring");
     ```
   - The vector's length becomes 4, and capacity remains 4 after adding these 4 elements.

5. **When Capacity Increases**:
   - If you try to add more elements than the current capacity, Rust will reallocate memory, double the capacity (typically), move the old elements to the new space, and add the new element.
     ```rust
     seasons.push("Summer"); // After this, the vector’s capacity expands.
     ```

6. **Memory Reallocation and Safety**:
   - **Dangling References**: If there are mutable or immutable references pointing to the old memory location (before reallocation), they become dangling references when the vector moves to the new location.
   - Rust prevents multiple mutable references to ensure memory safety and prevent bugs like dangling references.

7. **Why Rust Prevents Multiple Mutable References**:
   - When a vector repositions its data in memory (after capacity increases), any existing references that point to the old memory location would no longer be valid. Rust prohibits multiple mutable references to prevent issues like these from happening.

---

### **Key Takeaways**
- **Capacity and Memory Reallocation**: When a vector exceeds its capacity, Rust reallocates memory and moves data to a larger space.
- **Mutable Reference Safety**: Rust prevents multiple mutable references to avoid data corruption and dangling references when reallocation occurs.
- **Behind-the-Scenes Management**: The automatic management of vector capacity helps avoid memory issues while maintaining performance.
