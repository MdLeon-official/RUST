# The Copy Trait


#### 🔧 What is a Trait?
- A **trait** in Rust is like a **contract** that types can agree to follow.
- If a type **implements** a trait, it means it can **perform some behavior** that the trait defines.

---

#### 📋 The `Copy` Trait
- If a type implements the `Copy` trait, it means it can be **duplicated** (copied) easily and automatically.
- Rust will **automatically create a full copy** of the value when needed.

---

#### ✅ Types that implement `Copy`
- Most **primitive (fixed-size)** types implement the `Copy` trait:
  - `i32`, `f64`, `bool`, `char`, etc.

These are **stored on the stack**, and **copies are cheap and fast**.

---

#### 🧪 Example
```rust
fn main() {
    let time = 2025;
    let year = time; // <-- Copy happens here

    println!("Time: {}", time); // ✅ still valid
    println!("Year: {}", year); // ✅ also valid
}
```

- `time` is copied into `year`.
- Both variables **own their own value** (both 2025).
- ✅ Both can still be used after the copy.

---

#### 🔁 Stack Behavior
- Stack uses **LIFO** (Last In, First Out).
- So when `main()` ends:
  - `year` is removed first (last in).
  - Then `time`.

---

#### 🧠 Key Takeaways
- If a type has `Copy`, variables can be **duplicated** without invalidating the original.
- **No move happens**—just a simple copy.
- Only **stack-based, fixed-size types** have `Copy`.
- This won't work the same way for **heap-allocated data** (coming soon).
