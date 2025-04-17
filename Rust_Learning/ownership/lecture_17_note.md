# Return Values II


#### 🟠 Problem:
- Passing `String` or any owned value to a function **moves ownership** to that function.
- If we don’t return it, it's **deallocated** at the end of that function.
- We **lose access** to the original value unless we **return ownership back**.

#### 🧪 Example:
```rust
fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour. ");
    meal
}

fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    // add_sugar(current_meal) → must also return and reassign!
}
```

#### 😩 Why it’s annoying:
- Every function must:
  - Take ownership
  - Mutate (if needed)
  - **Return the String**
- Main function must:
  - **Reassign the result**
- With multiple functions (add_sugar, add_salt…), this becomes **repetitive and messy**.
- Even functions that only **read** (like print) must return if they take ownership.

#### 🧯 What happens if we don’t return?
- Ownership is lost.
- The heap memory is **deallocated**.
- Compiler will throw an error or it will silently drop the data.

#### 🛠 Workaround (bad way):
- Return everything.
- Use tuples if there are multiple owned parameters.

```rust
fn use_two_strings(a: String, b: String) -> (String, String) {
    // do something
    (a, b) // return both
}
```

#### 🧪 Result:
- This pattern works, but is **not scalable**.
- It's **boilerplate-heavy** and **hard to manage**.

---

### 🟢 Solution: Use **References (&)**

Rust gives us **references** to avoid moving ownership.
- They let us **borrow** the value without taking ownership.
- We'll use `&` (shared reference) or `&mut` (mutable reference).
