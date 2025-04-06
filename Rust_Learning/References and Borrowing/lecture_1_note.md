# Immutable and Mutable Reference Parameters


Imagine you're writing down a recipe (`String`) and passing it to a friend (a function) to read or edit.  
Now ask yourself:  
- Do they get the **original notebook**, or  
- Just **read it**, or  
- Can they **write** on it too?

That’s exactly what Rust is dealing with here.

---

### 🥄 The Problem: Ownership Transfer

When you pass a `String` to a function like this:

```rust
fn add_flour(meal: String) { ... }
```

You're **giving ownership** of that string to the function.  
Rust will clean it up after the function ends. So, to keep using it in `main()`, you must **return it**, which becomes annoying fast.

---

### ✅ The Fix: Use References

#### 1. **Immutable Reference** – Just read

```rust
fn show_my_meal(meal: &String) {
    println!("Meal steps: {}", meal);
}
```

- `&String` means **read-only access** to the String.  
- No need to return anything.  
- Ownership stays with `main()`.

📌 **Remember**: `&` = "I'm just borrowing your notebook to **read**."

---

#### 2. **Mutable Reference** – Can write

```rust
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}
```

- `&mut String` = reference with **write access**.  
- Ownership still stays in `main()`.  
- You can **change** the original value.

📌 **Remember**: `&mut` = "I'm borrowing your notebook to **write** on it."

---

### ⚠️ Important: Match Types!

If the function expects a `&mut String`, you must pass one:

```rust
let mut current_meal = String::new();
add_flour(&mut current_meal);  // ✅
```

Passing `&current_meal` will cause an error because it's immutable.

---

### 🔁 4 Options for Function Parameters

| Parameter Type     | Ownership | Mutable? | Use Case                      |
|--------------------|-----------|----------|-------------------------------|
| `meal: String`     | Yes       | No       | Takes ownership, no changes   |
| `mut meal: String` | Yes       | Yes      | Takes ownership, can modify   |
| `meal: &String`    | No        | No       | Borrow to read only           |
| `meal: &mut String`| No        | Yes      | Borrow to read & write        |

---

🧠 **Tip to Remember**:
- `String` → Owns the data.
- `&String` → Borrows to read.
- `&mut String` → Borrows to edit.
