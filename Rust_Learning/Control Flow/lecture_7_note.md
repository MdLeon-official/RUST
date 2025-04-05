# The match Statement with Multiple Values and Conditionals


### 🔁 Matching Multiple Values (using `|`)
```rust
match number {
    2 | 4 | 6 | 8 => println!("{number} is even"),
    1 | 3 | 5     => println!("{number} is odd"),
    _             => println!("Unknown for now"),
}
```
- `|` acts like a logical **OR** — matches if `number` is any of the listed values.
- `_` is a **wildcard** that matches everything else.

---

### 🛡 Pattern Matching with Guards (`if`)
```rust
match number {
    value if value % 2 == 0 => println!("{value} is an even number"),
    x     if x % 2 != 0     => println!("{x} is an odd number"),
    _                       => println!("Unknown for now"), // fallback arm
}
```
- The variable (`value` or `x`) is **bound** temporarily during the match.
- The `if` condition acts as a **guard**: it filters the match even further.

---

### ❌ Replacing unreachable arms
```rust
_ => unreachable!(),
```
- `unreachable!()` is used when **you know** a pattern will never be reached, but the compiler doesn’t.
- It’s cleaner than a dummy `println!`.

---

### 🧠 Why use pattern guards?
They make match arms **more expressive** and **flexible** than just matching on raw values — especially useful when you want to apply **logic** instead of a hardcoded list.
