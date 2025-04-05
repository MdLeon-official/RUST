# While Loop


### ✅ **What is a `while` loop?**
- **Repeats** a block of code while a **condition is true**.
- **Automatically terminates** when the condition becomes false.

### 🧭 **Basic Syntax of `while` loop**
```rust
while condition {
    // code to execute while the condition is true
}
```
- `condition`: a boolean expression (e.g., `seconds > 0`).

### 🚫 **Automatic Termination**
- No need for a `break` keyword; the loop **terminates automatically** once the condition is false.

### 🔁 **Emulating the Countdown with `while` Loop**

```rust
fn main() {
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds");
            seconds -= 3;
            continue; // skip to next iteration
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }

    println!("Blastoff!"); // Output once the loop terminates
}
```

---

### 🧠 **Key Points**
- The `while` loop **automatically exits** when the condition is false.
- The `continue` keyword can still be used to **skip the rest of the loop** and start the next iteration.
- **Condition modification** (e.g., updating `seconds`) is crucial for the loop to eventually stop.
