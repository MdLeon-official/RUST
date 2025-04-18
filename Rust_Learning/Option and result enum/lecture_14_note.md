# The while let Construct

#### ✅ `if let` — Conditional Pattern Matching
- Used to **extract data** from an enum **only once**, if it matches a pattern (like `Some(value)`).
- Syntax:
  ```rust
  if let Some(sauce) = sauces.pop() {
      println!("The next sauce is {}", sauce);
  }
  ```

#### 🔁 `while let` — Loop with Pattern Matching
- Repeats as long as a pattern keeps matching.
- Great for **looping through a vector** or similar structure **until empty**.
- Syntax:
  ```rust
  while let Some(sauce) = sauces.pop() {
      println!("The next sauce is {}", sauce);
  }
  ```

#### ⚙️ How it works:
- Left-hand side: the **pattern** (e.g., `Some(sauce)`)
- Right-hand side: the **dynamic expression** (e.g., `sauces.pop()`)
- Loop continues as long as the **right-hand side evaluates to the left-hand pattern**.
- Stops when it hits a `None`.

#### 📌 Real Use Case Example:
```rust
fn main() {
    let mut sauces = vec!["Ranch", "Ketchup", "Mayonnaise"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {}", sauce);
    }
}
```

#### Why it’s useful:
- **Cleaner than using multiple `if let`**
- Avoids unnecessary repetition
- Helps when consuming items from a `Vec`, `Option`, or custom enum
