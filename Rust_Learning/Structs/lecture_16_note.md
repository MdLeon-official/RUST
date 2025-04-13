# Tuple Structs



#### 🧱 **Definition**
- A **tuple struct** is a struct without named fields.
- Syntax resembles a tuple, but it has a **struct name**, giving it a **distinct type**.

```rust
struct ShortDuration(u32, u32); // Represents (hours, minutes)
struct LongDuration(u32, u32);  // Represents (years, months)
```

---

#### 🛠️ **Usage / Instantiation**
```rust
let work_shift = ShortDuration(8, 0);
let era = LongDuration(5, 3);
```

---

#### 🔍 **Accessing Fields**
- Use dot + index (just like tuples):

```rust
println!("{} hours and {} minutes", work_shift.0, work_shift.1);
println!("{} years and {} months", era.0, era.1);
```

---

#### ✅ **Advantages Over Tuples**
- Even if the inner types are the same, **each tuple struct is a unique type**.
- Prevents bugs: compiler enforces type safety.

```rust
fn go_to_work(length: ShortDuration) {
    println!("Passing time: {} hours, {} minutes", length.0, length.1);
}

// ✅ Valid
go_to_work(work_shift);

// ❌ Compiler error
go_to_work(era); // LongDuration is a different type!
```

---

#### ⚠️ **Regular Tuple Pitfall**
- `(u32, u32)` is treated the same everywhere.
- No type distinction between `(8, 0)` and `(5, 3)` → leads to accidental misuse.

```rust
fn go_to_work(length: (u32, u32)) {
    println!("{} hours, {} minutes", length.0, length.1);
}

// ❌ Both accepted, even if wrong
go_to_work(work_shift); // No error!
go_to_work(era);        // Still no error!
```

---

#### 💡 **Remember**
- Tuple structs give structure and **type distinction** without needing field names.
- Ideal for lightweight, positionally-identified data with semantic meaning.
