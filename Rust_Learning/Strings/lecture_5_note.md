# Collecting User Input with read_line Method


...removing that extra newline `\n` character from the end of the user's input, but also because it cleans up any leading or trailing spaces they might have accidentally added. This gives us a nice clean string to work with.

So instead of this:

```rust
println!("Hello {}", name);
```

We can write:

```rust
println!("Hello {}", name.trim());
```

That way, even though `read_line` includes the newline character when capturing input, the `trim` method will take care of it — making the output cleaner and more user-friendly.

---

### 🔑 **Quick Summary:**

- Use `std::io` for input/output operations in Rust.
- Use `io::stdin().read_line(&mut string)` to read user input.
- The `read_line` method:
  - Appends the user input (including newline `\n`) to the string.
  - Returns a `Result` enum (`Ok(bytes_read)` or `Err(e)`).
- Handle `Result` with `.expect()` or `match`:
  - `.expect("error message")` crashes on error with a message.
  - `match` allows custom behavior for success and failure.
- Use `.trim()` to remove trailing newline or spaces from input.

---

### 🧪 Full Working Example:

```rust
use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Hello {}", name.trim());
        }
        Err(message) => {
            println!("There was an error: {}", message);
        }
    }
}
```
