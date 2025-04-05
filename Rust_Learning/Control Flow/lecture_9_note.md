# The continue Keyword

- Skips the **rest of the current loop iteration**.
- Jumps back to the top of the loop immediately.
- Useful when you want to conditionally **bypass remaining code**.

```rust
if seconds % 2 == 0 {
    println!("{seconds} (even number), skipping 3 seconds");
    seconds -= 3;
    continue; // skip remaining logic for this loop
}
```

---

### 🧨 **Complete Example: Countdown with `loop`, `break`, and `continue`**

```rust
fn main() {
    let mut seconds = 21;

    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds");
            seconds -= 3;
            continue; // go to next iteration, skip below
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
}
```

---

### 💡 Key Takeaways
- `loop {}` = infinite repetition.
- `break` = full exit from loop.
- `continue` = skip rest of current iteration, go to next one.
