# Define a Struct


### 🟤 What is a `struct`?
- A **struct** (short for "structure") is a **container** that holds related data together.
- Similar to **objects** in object-oriented programming.

### 🟤 Why not just use tuples?
Tuples can hold different types of values like:
```rust
("Latte", 5.99, true)
```
But:
- ❌ No context → What does `true` or `5.99` mean?
- ❌ Order matters (but it shouldn’t)

> ✅ We need **clear names**, not just positions → that's where **structs** help.

---

### 🟤 Defining a Struct (Named Fields)
Structs have **named fields**, so it's clear what each value means.

Example:
```rust
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}
```

💡 Think of it as a **blueprint**, like saying:
- "A *Coffee* has a name, price, and hot/cold status."

> ⚠️ This doesn’t create a coffee yet — just tells Rust *what a Coffee looks like*.

---

### 🟤 Naming Rules
- **Struct names** → `PascalCase` → e.g., `CoffeeDrink`
- **Field names** → `snake_case` → e.g., `is_hot`

🧠 **Remembering Tip**:
> "Structs are like blueprints. You define the *shape*, not the actual item."
