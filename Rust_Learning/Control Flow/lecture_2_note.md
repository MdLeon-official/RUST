# The else if Statement


### ✅ `if` Statement

```rust
if true {
    println!("This runs");
}

if false {
    println!("This doesn't run");
}
```

- Only runs the block if the condition is `true`.
- No parentheses needed (unlike C/C++).
- The condition **must be a Boolean** (`true` or `false`) — no "truthiness" like in Python/JS.

---

### ⚠️ Condition from Variable

```rust
let some_condition = true;

if some_condition {
    println!("This runs if condition is true");
}
```

- Usually, the condition is not hardcoded. It comes from a variable, user input, or some calculation.

---

### 🌿 `else if` and `else`

Used for **related checks** — Rust evaluates them **in order** and executes **only the first true** condition.

```rust
let season = "fall";

if season == "summer" {
    println!("School's out");
} else if season == "winter" {
    println!("Brr, so cold!");
} else if season == "fall" {
    println!("Leaves falling");
} else if season == "spring" {
    println!("Lots of rain");
}
```

- Rust **stops** at the first true condition.
- If none are true, **nothing runs** (unless you use a final `else` block).

---

### 🔄 Multiple `if` vs. `else if`

```rust
// These are independent checks
if season == "summer" {
    println!("School's out");
}
if season == "fall" {
    println!("Leaves falling");
}
```

- All `if` blocks are evaluated.
- Use this when conditions are **unrelated** or **multiple can be true**.

---

### ✅ Best Practice

- Use `if` + `else if` when your conditions are **mutually exclusive** and **related**.
- Use separate `if`s for **independent** logic.
