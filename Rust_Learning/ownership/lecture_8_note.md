# The drop Function



### 📦 What is a **Move**?
- A **move** is when **ownership of a value** transfers from one variable to another.
- After the move, the **original variable becomes invalid**.
- This helps prevent **double free** errors (trying to free the same memory twice).

### 🧮 Example 1: With Integers (Stack Data)
```rust
let time = 2025;
let year = time; // 'year' is a copy of 'time'
```
- `i32` is **Copy**, so a full copy is made.
- Both `time` and `year` are valid.

### 📚 Example 2: With Strings (Heap Data)
```rust
let person = String::from("Boris");
let genius = person; // ownership moves to 'genius'
```
- `String` is **not Copy**, so no duplicate heap data.
- After this, `person` is **invalid**, only `genius` owns the String.

### ❌ Invalid Example:
```rust
println!("{}", person); // ERROR: borrow of moved value
```

---

### 🧹 `drop`: Rust's Cleanup Function
- Rust calls `drop(variable)` **automatically** when a variable **goes out of scope**.
- It’s used to **deallocate heap memory**.

#### Example:
```rust
let person = String::from("Boris");
drop(person); // manually drops it

println!("{}", person); // ❌ ERROR: 'person' is invalid
```

- Once you `drop` a variable, you **can’t use it** or **move** it again.

---

### ✅ Key Rules:
1. Only **one owner** at a time for heap data.
2. **Move** = transfer ownership.
3. **Invalid** means: you can’t use or move it anymore.
4. Rust handles cleanup automatically using `drop`.
5. Stack values (like integers) don’t use `drop`.
