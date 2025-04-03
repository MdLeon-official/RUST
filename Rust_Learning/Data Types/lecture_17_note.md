# Reading and Writing Array Elements

- **Indexing**: Arrays in programming typically start indexing from 0, not 1. This means that the first element of an array is at index 0, the second at index 1, and so on.
  - Example: For the array `["Spring", "Summer", "Fall", "Winter"]`, the indices would be:
    - Spring: 0
    - Summer: 1
    - Fall: 2
    - Winter: 3
- **Accessing Elements**: To access an element, you provide the array's name followed by the index in square brackets, e.g., `seasons[0]` to access "Spring".
- **Invalid Index Access**: Trying to access an index that doesn't exist (e.g., `seasons[100]` when the array has only 4 elements) will result in a runtime panic.
- **Mutable Arrays**: Arrays can be made mutable by using the `mut` keyword, which allows you to replace an element at a given index. However, you cannot change the size of the array itself (i.e., you can't add or remove elements).

### Example Code:
```rust
fn main() {
    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];
    
    // Accessing and printing elements by index
    println!("First season: {}", seasons[0]); // "Spring"
    println!("Second season: {}", seasons[1]); // "Summer"
    
    // Replacing an element in the array
    seasons[2] = "Autumn";  // Changing "Fall" to "Autumn"
    
    // Printing the updated value
    println!("Updated third season: {}", seasons[2]); // "Autumn"
}
```
