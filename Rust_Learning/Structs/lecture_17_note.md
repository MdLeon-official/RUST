# Unit-Like Structs


#### 🧱 What is a unit?
- A **unit** is a tuple with no values: `()`
- It acts as **both a type and a value**

#### 🧱 What is a unit-like struct?
- A struct **with no fields** or data.
- Declared like this:
  ```rust
  struct Empty;
  ```

#### ✅ Instantiating a unit-like struct:
```rust
let my_empty_struct = Empty;
```
- No `{}` like named-field structs
- No `()` like tuple structs
- Just the struct name

#### 💡 Why use it?
- May seem pointless at first (no data)
- But:
  - You **can define methods** with `impl`
  - Useful in some **design patterns** and **marker traits**
  - Helps in creating types with **behavior but no state**
