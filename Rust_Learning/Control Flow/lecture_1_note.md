# The if Statement


**Control flow** = how your program decides **what to do next**. Instead of always running from top to bottom, we can tell Rust:  
👉 “Only run this part **if** something is true.”

---

### 🔑 `if` Statement

```rust
if condition {
    // code here runs only if condition is true
}
```

- `condition` **must** be a Boolean (`true` or `false`)
- ✅ No parentheses needed (like `if true` — **not** `if (true)`)
- ❌ Rust does **not** support “truthy” values (e.g. numbers like 5 won't work)

---

### ✅ Example

```rust
let should_run = true;

if should_run {
    println!("This line will be printed!");
}

if false {
    println!("This line will NOT be printed!");
}
```

---

### 🔄 Real Life Example

Imagine a **Pause button** on a video player:

- "If user presses pause → then stop the video."
- It doesn’t pause by itself — it checks **if a condition is met**.

Just like that, Rust uses `if` to check a condition before doing something.

---

### 💡 Remember

> ❗**Only Booleans allowed**  
> ✅ `if 5 > 3 {}` → OK  
> ❌ `if 5 {}` → Error

Rust wants **clear logic** — true or false, no guesswork.
