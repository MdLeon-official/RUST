# self Parameter as Mutable Struct Instance



### 🔧 Four Ways to Define `self` in Methods

1. `&self` → Borrow immutably (read-only access, no ownership)
2. `&mut self` → Borrow mutably (change allowed, no ownership)
3. `self` → Takes ownership (read-only access, full ownership)
4. `mut self` → Takes ownership **and** allows mutation (this lesson)

---

### ✅ What This Lesson Demonstrates

#### 1. **Method Name**: `double_length`
- Purpose: Double the song’s duration and update the field in the struct.

#### 2. **Definition**
```rust
fn double_length(mut self) {
    self.duration_secs = self.duration_secs * 2;
    println!("{:#?}", self);  // Pretty-print the updated struct
}
```

- `mut self`: This gives the method full ownership *and* permission to mutate the struct.
- You can access and modify fields like `self.duration_secs`.
- Uses `println!("{:#?}", self)` — the `#?` format prints the struct using the `Debug` trait.

---

### 📦 Important Notes

- When you use `self` or `mut self`, you're **taking ownership**.
- That means the original variable (like `song`) **can’t be used again** after calling the method unless you **return self**.
- For example:
  ```rust
  song.double_length();     // ✅ works
  song.display_song_info(); // ❌ error, song was moved
  ```

---

### 🧠 Extra Tip

- `Self` (capital S) is an alias for the type you're implementing (`TaylorSwiftSong` in this case).
- More robust if the struct name changes later.

---

### 💡 When to Use `mut self`
Use it when:
- You want to **change the struct fields**.
- You’re okay with **consuming the instance** (it won’t be usable after calling the method).

