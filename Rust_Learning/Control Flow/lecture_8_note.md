# The loop and break Keywords


### 🔹 What is Iteration?
- **Iteration** means repeating a block of code multiple times.
- Useful for automating tasks and processes.

### 🔹 `loop` Keyword
- The simplest way to create a loop in Rust.
- Syntax:
  ```rust
  loop {
      // code runs forever unless manually stopped
  }
  ```

### ⚠️ Infinite Loop
- A `loop` without a stop condition runs **forever**, which can crash your system or IDE.
  ```rust
  loop {
      println!("Running");
  }
  ```

### 🔹 Stopping a Loop with `break`
- `break` exits the loop immediately.
- Usually used with an `if` condition.

---

## ⏱ Example: Countdown Timer

```rust
fn main() {
    let mut seconds = 10;

    loop {
        if seconds == 0 {
            println!("Blastoff!");
            break;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
}
```

### 🔍 Explanation:
- `let mut seconds = 10;` → A mutable variable to track time.
- `seconds -= 1;` → Decreases `seconds` by 1 each iteration.
- `if seconds == 0` → When `seconds` becomes 0:
  - Prints `"Blastoff!"`
  - Exits loop with `break`.

---

### ✅ Summary:
- `loop {}` repeats code forever unless stopped.
- Use `break` to exit the loop.
- Pair `break` with `if` to stop based on a condition.
- Make variables **mutable** (`mut`) if their values change inside the loop.
