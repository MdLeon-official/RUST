# String Slice Lengths



#### 🔸 String Slice Length is in **Bytes**, Not Characters

- When you use `.len()` on a string slice (`&str`), it returns the **number of bytes**, not the number of visible characters.
  
  ```rust
  let food = "pizza"; // ASCII, each char = 1 byte
  println!("{}", food.len()); // 5
  ```

- If you slice it:
  
  ```rust
  let pizza_slice = &food[0..3]; // "piz"
  println!("{}", pizza_slice.len()); // 3
  ```

---

#### ⚠️ Unicode & Multi-byte Characters (e.g., Emojis)

- Some characters (like emojis) take **more than one byte**.

  ```rust
  let food = "🍕";
  println!("{}", food.len()); // 4 bytes
  ```

- Trying to slice such characters incorrectly causes a **runtime panic**:

  ```rust
  let slice = &food[0..3]; // ❌ PANIC: not a valid character boundary
  ```

- ✅ The only valid slice is one that includes the **full byte range** of that character:

  ```rust
  let slice = &food[0..4]; // ✅ OK
  ```

---

### 🧠 Key Takeaways

- `&str` (string slice) is a **reference to bytes**, not to characters.
- Always slice at **valid character boundaries** (i.e., not in the middle of a multi-byte character).
- Regular English letters (ASCII) are safe: 1 byte per character.
- Unicode (e.g., emojis) = multi-byte → be cautious when slicing!
