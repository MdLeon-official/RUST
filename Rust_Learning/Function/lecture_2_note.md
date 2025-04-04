# Parameters and Arguments

### 🔹 Definitions:
- **Parameter**: A named, expected input declared in a function’s definition.
- **Argument**: The actual, concrete value passed to a function when it’s called.

### 🔹 Syntax:
```rust
fn function_name(parameter_name: type) {
    // use parameter_name like a variable
}
```

### 🔹 Example 1: Single Parameter
```rust
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

open_store("Brooklyn");
open_store("Queens");
```

💡 `"Brooklyn"` and `"Queens"` are **arguments** passed for the **neighborhood** parameter.

---

### 🔹 Example 2: Multiple Parameters
```rust
fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

bake_pizza(20, "pepperoni");
bake_pizza(15, "mushroom");
```

💡 `20` and `15` are **arguments** for `number`; `"pepperoni"` and `"mushroom"` are arguments for `topping`.

---

### 🔹 Key Rules:
- Separate multiple parameters with a comma.
- Provide both **name** and **type** for each parameter.
- Arguments must match the number and **type** of parameters.

### ⚠️ Compiler checks for:
- Correct number of arguments.
- Correct **type** of each argument.
