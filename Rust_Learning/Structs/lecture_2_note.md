# Create a Struct Instance


- After defining a `struct`, we can **create instances** (real values) of that type.
- Syntax:
  ```rust
  let variable_name = StructName {
      field1: value1,
      field2: value2,
      field3: value3,
  };
  ```

#### ✅ Example:
```rust
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
}
```

#### ⚠️ Rules:
- All fields **must be provided**.
- Field **names must match** exactly.
- Field **types must match** the struct definition.
- **Order of fields doesn’t matter** since they're named.

#### ❌ Errors You May Encounter:
- Missing field: `missing field 'price'`
- Wrong type: `expected bool, found integer`
- Extra fields: `structure has no field named 'xyz'`
