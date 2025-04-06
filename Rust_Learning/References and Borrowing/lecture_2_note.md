# Multiple Immutable References


#### 🔹 Why Rust Cares About References
- A **reference** lets you **access data without copying** it.
- But if two references **expect different values** and one of them changes the data, the program can become **buggy** or **unpredictable**.

#### 🔹 The Car Analogy 🚗
- You own a **blue car** (`String` data).
- Two friends ask to **borrow** it (create references).

1. **Immutable Borrow** (Friend #1):
   - Promises **not to change** the car.
   - Expects a **blue car back**.

2. **Mutable Borrow** (Friend #2):
   - Allowed to **modify** the car (e.g., repaint to red).
   - First friend gets upset if they get back a **red car**.

#### ⚠️ The Problem
If both borrows exist at the same time:
- One expects **blue**, the other returns **red** → conflict.
- In many languages, this leads to bugs.
- 🦀 **Rust prevents this entirely at compile time.**

---

### ✅ Rust's Rules (Important!)

| Rule | Description |
|------|-------------|
| ✔️ Multiple Immutable References Allowed | As many as you want. No risk, because they **can’t change** data. |
| ❌ Mutable + Immutable References at the Same Time | **Not allowed**. Prevents data races and bugs. |
| ✔️ One Mutable Reference Allowed Alone | Only one mutable reference at a time is safe. |

#### 🗣️ "Any number of readers, only one writer."

---

### 🧪 Rust Code Example

```rust
fn main() {
    let car = String::from("Red");

    let ref1 = &car;        // Immutable reference
    let ref2 = &car;        // Another immutable reference

    println!("{}, {}, {}", ref1, ref2, &car);
}
```

- All good: ✅
  - No mutation
  - All references are **immutable**
  - No surprises: all point to `"Red"`

---

### 🧠 Remember:
- Immutable references = Safe for **reading**
- Mutable references = Safe for **changing**
- Mixing = **Not allowed** (Rust prevents it)
