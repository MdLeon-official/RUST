# Mutable Array Slices




1. **Mutable Array**: 
   You start by creating a mutable array that allows modification:
   ```rust
   let mut my_array = [10, 15, 20, 25, 30];
   ```

2. **Immutable Slice**: 
   By default, when you create a slice, it’s immutable:
   ```rust
   let my_slice = &my_array[2..4]; // Immutable slice of elements [20, 25]
   ```

3. **Mutable Slice**:
   To mutate a portion of an array, you need to declare a mutable slice:
   ```rust
   let mut my_slice = &mut my_array[2..4]; // Mutable slice
   ```

4. **Modifying the Slice**:
   After obtaining a mutable slice, you can modify elements within the slice, which affects the original array:
   ```rust
   my_slice[0] = 100; // Modify the first element of the slice, which is part of the original array
   ```

5. **Impact on Original Array**:
   Modifying the mutable slice will mutate the original array since the slice is just a reference to a portion of it:
   ```rust
   println!("{:?}", my_array); // Prints: [10, 15, 100, 25, 30]
   ```

### Notes:
- **Slices** are just references to parts of an array. You can create **immutable slices** to read data, but to **modify** the data, you need to create a **mutable slice**.
- Mutating a **mutable slice** affects the original array because the slice doesn’t own the data; it borrows it.

### Differences Between String Slices and Array Slices:
- **String slices** (`&str`) are immutable in Rust; you cannot modify them.
- **Array slices** (`&[T]` or `&mut [T]`) can be either immutable or mutable, depending on whether you use `mut` to borrow them.
