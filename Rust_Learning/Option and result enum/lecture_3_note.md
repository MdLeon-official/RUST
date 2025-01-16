## The unwrap and expect method: 

### **Extracting Values from `Option`**

The `Option` type in Rust wraps a core value, which needs to be explicitly extracted. Two common methods for this are `unwrap` and `expect`. While these methods are straightforward, they can lead to runtime panics if the `Option` contains `None`.

---

### **Using `unwrap`**

- **Purpose**: Extract the value inside the `Some` variant of an `Option`.
- **Behavior**: 
  - Returns the inner value if the `Option` is `Some`.
  - Panics if the `Option` is `None`.

#### Code Example:

```rust
fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // Valid case
    let bass = musical_instruments.get(2);
    let valid_instrument = bass.unwrap(); // Extracts "Bass"
    println!("{}", valid_instrument); // Output: Bass

    // Invalid case (causes panic)
    let invalid_instrument = musical_instruments.get(100);
    // invalid_instrument.unwrap(); // Panics: called `unwrap()` on a `None` value
}
```

- **Risk**: Using `unwrap` blindly assumes the presence of a value, which can result in runtime panics when the `Option` is `None`.

---

### **Using `expect`**

- **Purpose**: Similar to `unwrap`, but allows you to provide a custom error message.
- **Behavior**: 
  - Returns the inner value if the `Option` is `Some`.
  - Panics with the custom message if the `Option` is `None`.

#### Code Example:

```rust
fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // Valid case
    let bass = musical_instruments.get(2);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{}", valid_instrument); // Output: Bass

    // Invalid case (causes panic with custom message)
    let invalid_instrument = musical_instruments.get(100);
    invalid_instrument.expect("Unable to retrieve musical instrument"); 
    // Panic: Unable to retrieve musical instrument
}
```

- **Advantage over `unwrap`**: Provides better debugging context through custom error messages.
- **Same Risk**: Like `unwrap`, it panics if the `Option` contains `None`.

---

### **Key Takeaways**

1. **Risk of Panic**:
   - Both `unwrap` and `expect` can lead to runtime panics if used on `None`.
   - Use these methods only when you're certain the `Option` contains `Some`.

2. **Custom Error Messages**:
   - `expect` is preferred over `unwrap` when debugging or explaining errors, as it allows custom messages.

3. **Not Idiomatic**:
   - While these methods are convenient, they are not considered the safest or most idiomatic way to handle `Option`.
   - Rust encourages developers to explicitly handle both `Some` and `None` variants.

