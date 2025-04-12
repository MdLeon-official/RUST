# Passing Structs into a Function



#### **1. Pass by Value (Immutable)**
```rust
fn drink_coffee(coffee: Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}
```
- 🔁 **Ownership moves** to the function.
- 📦 Cannot access `mocha` after calling `drink_coffee(mocha)`.
- 🛠️ Cannot **mutate** fields.
- ✅ Use when the function should consume the value.

---

#### **2. Pass by Value (Mutable)**
```rust
fn drink_coffee(mut coffee: Coffee) {
    coffee.is_hot = false;
}
```
- 🔁 **Ownership still moves**.
- 🔧 Can **mutate** fields.
- 📦 Still can't access `mocha` after the function.
- 🔄 Need to **return** the struct if you want to use it later.
- ✅ Use if the function both **owns and modifies** the struct.

---

#### **3. Pass by Reference (Immutable)**
```rust
fn drink_coffee(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}
```
- 🎯 **No ownership moved**, just borrowed.
- 🛠️ Cannot mutate fields.
- ✅ Use if the function just **reads** from the struct.

---

#### **4. Pass by Reference (Mutable)**
```rust
fn drink_coffee(coffee: &mut Coffee) {
    coffee.price = 10.99;
}
```
- 🔒 **No ownership moved**.
- 🔧 Can **mutate** fields.
- 🧠 Must declare original variable as `mut`.
- ✅ Use if you want to **modify** but **not take ownership**.

---

### 📝 When to Use What?

| Method                     | Ownership | Mutates? | Best For                        |
|---------------------------|-----------|----------|---------------------------------|
| `fn f(s: Coffee)`         | Yes       | No       | Consuming a struct              |
| `fn f(mut s: Coffee)`     | Yes       | Yes      | Modifying + consuming           |
| `fn f(s: &Coffee)`        | No        | No       | Reading data only               |
| `fn f(s: &mut Coffee)`    | No        | Yes      | Modifying without ownership     |
