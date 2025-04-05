# The else Statement



The `else` block is a **fallback** – it **runs when none** of the previous `if` or `else if` conditions are true.

---

### 🔍 Syntax Example:

```rust
let season = "spring";

if season == "summer" {
    println!("School's out");
} else if season == "winter" {
    println!("Brr, so cold!");
} else {
    println!("Lots of rain"); // For spring, fall, or anything else
}
```

---

### 🧠 Key Points to Remember:

- `else` has **no condition**.
- Always comes **at the end** of the chain.
- It runs **only if all prior `if` and `else if` are false**.
- Makes sure **one block always executes**.

---

### 💡 Minimal Usage (Just `if` + `else`):

```rust
let is_raining = true;

if is_raining {
    println!("Take an umbrella");
} else {
    println!("Enjoy the sun");
}
```

- One of the blocks **will always run**.
- Useful when you want **only two outcomes**.

---

### ✅ Recap:

| Keyword   | Needs Condition? | Must Follow?   | Purpose                        |
|-----------|------------------|----------------|--------------------------------|
| `if`      | ✅ Yes            | –              | Start condition check          |
| `else if` | ✅ Yes            | `if` or another `else if` | Additional condition |
| `else`    | ❌ No             | After `if` or `else if`  | Final fallback logic           |
