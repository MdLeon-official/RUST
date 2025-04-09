# Array Slices


## What is a Slice?

A **slice** is a **reference to a portion** of a collection (like an array or `String`). It borrows a chunk of memory without taking ownership. This allows you to work with just a part of the data efficiently.

---

## 📚 Key Differences: `&[T]` vs `&[T; N]`

| Concept                            | Slice (`&[T]`)                          | Array Reference (`&[T; N]`)                |
|------------------------------------|----------------------------------------|-------------------------------------------|
| Refers to                          | A **portion** of an array              | The **entire** array                      |
| Length Included in Type?           | ❌ (Dynamic)                            | ✅ (Static – part of the type)             |
| Flexible (accepts varying lengths) | ✅                                      | ❌ (only matches arrays of length N)       |
| Example Type                       | `&[i32]`                                | `&[i32; 6]`                                |
| Use Case                           | For function params accepting slices   | When you need to know the exact length    |

---

## 🔢 Syntax to Create Array Slices

```rust
let values = [4, 8, 15, 16, 23, 42];

// Slice of first 3 elements
let slice = &values[0..3]; // 4, 8, 15

// Same as above: start omitted
let slice = &values[..3];  // 4, 8, 15

// Slice from middle
let slice = &values[2..4]; // 15, 16

// Slice to the end
let slice = &values[2..];  // 15, 16, 23, 42

// Full slice of array (still a slice)
let slice = &values[..];   // full array as a slice (&[i32])

// Full reference (not a slice)
let full_ref = &values;    // type: &[i32; 6]
```

---

## ✅ Why Prefer Slices in Functions?

Because they are **more flexible**:

```rust
fn print_numbers(slice: &[i32]) {
    for number in slice {
        println!("{}", number);
    }
}
```

This function works with any length slice:

```rust
let values = [1, 2, 3, 4, 5, 6];
print_numbers(&values[1..4]); // ok
print_numbers(&values);       // ok because Rust auto-coerces &[i32; 6] → &[i32]
```

But if you had:

```rust
fn print_exact(nums: &[i32; 6]) { ... }
```

Then you'd **only** be able to pass in 6-element arrays—not slices, not arrays of length 5 or 7.

---

## 🧠 Remember This

- `&String` → **reference to the entire String**
- `&str` → **string slice**, more versatile
- `&[i32; N]` → reference to an array of **exact** length N
- `&[i32]` → **array slice**, more flexible
