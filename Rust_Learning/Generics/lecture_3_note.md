## Multiple Generics


### **1. Combining a Generic Type and a Concrete Type**
You can mix a generic type with a concrete type in function parameter declarations. For instance:

```rust
fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}
```

- **`T`** is a generic type that provides flexibility for the `first` parameter.
- The `second` parameter is fixed as an `i32`.
- The return value is a tuple `(T, i32)`.

#### Example:
```rust
let result = make_tuple("hello", 5); // Compiles: `T` is a string slice (&str)
let result = make_tuple("world", "not an integer"); // Error: second parameter must be i32
```

---

### **2. Reusing the Same Generic Type for Multiple Parameters**
You can use the same generic type for multiple parameters to enforce that they must be the same type:

```rust
fn make_tuple<T>(first: T, second: T) -> (T, T) {
    (first, second)
}
```

- Both `first` and `second` share the same type `T`.
- Rust enforces that both parameters must have the same type in each function invocation.

#### Example:
```rust
let result = make_tuple(5, 10); // Compiles: both are i32
let result = make_tuple("hello", "world"); // Compiles: both are &str
let result = make_tuple(5, "hello"); // Error: mismatched types
```

---

### **3. Declaring Multiple Generic Types**
You can declare multiple generic types using commas and use them for different parameters:

```rust
fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
```

- `T` and `U` are independent generics, allowing `first` and `second` to have different types.
- The return value is a tuple `(T, U)`.

#### Example:
```rust
let result = make_tuple(42, "hello"); // Compiles: T = i32, U = &str
let result = make_tuple(true, 3.14); // Compiles: T = bool, U = f64
let result = make_tuple(5, 10); // Compiles: T = i32, U = i32
```

---

### **Key Comparisons**

| **Scenario**                          | **Syntax**                                    | **Constraints**                                                                                     |
|---------------------------------------|----------------------------------------------|----------------------------------------------------------------------------------------------------|
| Generic + Concrete Type               | `fn make_tuple<T>(first: T, second: i32)`    | `first` is flexible, `second` must be `i32`.                                                      |
| Single Generic for All Parameters     | `fn make_tuple<T>(first: T, second: T)`      | Both parameters must be of the same type `T`.                                                     |
| Multiple Generics for Parameters      | `fn make_tuple<T, U>(first: T, second: U)`   | Parameters can have different types `T` and `U`, but may also be the same type (e.g., `T = U`).    |

---

### **Flexibility and Validation**
- Rust uses the provided arguments to infer the concrete types for generics (`T`, `U`) at runtime.
- With multiple generics, there’s no requirement for types to differ, but the possibility for them to differ adds flexibility.

#### Examples:
```rust
let mixed = make_tuple::<bool, f64>(true, 3.14); // Explicit type annotations
let same_type = make_tuple(10, 20);             // Same type for T and U
let another = make_tuple("Rust", 2025);         // Different types for T and U
```

---

### **Summary**
- Generics enable reusable, type-safe functions.
- Mixing generics and concrete types allows selective flexibility.
- Reusing the same generic creates stricter constraints.
- Multiple generics increase flexibility but still allow same-type parameters.

Rust enforces these constraints through the type system, ensuring correctness at compile time!
