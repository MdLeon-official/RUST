# Ownership and Function Parameters


### 📥 When You Pass a Variable to a Function

Rust checks:
- Does the type **implement `Copy`**?  
  → If **yes**, the value is **copied**  
  → If **no**, the value is **moved** (ownership changes)

---

### ✅ Example 1: Copy Type (like `i32`, `bool`, `f32`)

```rust
fn print_my_value(value: i32) {
    println!("Your value is {value}");
}

fn main() {
    let apples = 6;
    print_my_value(apples);
    println!("{apples}"); // apples is still valid!
}
```

🔑 **Why?**  
`i32` implements the `Copy` trait → the function gets a **copy**, not ownership.

---

### ❌ Example 2: Non-Copy Type (like `String`)

```rust
fn print_my_value(value: String) {
    println!("Your value is {value}");
}

fn main() {
    let oranges = String::from("oranges");
    print_my_value(oranges);
    println!("{oranges}"); // ❌ ERROR! oranges was moved
}
```

🔑 **Why?**  
`String` does **not** implement `Copy` → ownership is **moved** to the function.

---

### 💡 Remember Tip:
> Passing a value to a function is like **giving it to a friend**.  
> - If it's **Copy-able**, it's like giving a **photocopy** (you keep yours).  
> - If it's **not Copy-able**, it's like giving the **only copy** (you lose it).

---

### 🛠️ Fixing Move Errors:
To avoid ownership transfer, use `.clone()`:

```rust
print_my_value(oranges.clone());
```
