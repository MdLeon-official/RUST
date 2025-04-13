# Multiple impl Blocks


#### 📌 What’s the idea?
- You can **split your methods and associated functions** across **multiple `impl` blocks**.
- All `impl` blocks targeting the same type are **merged by Rust** into one logical implementation.

---

### ✅ Why is it useful?
- No functional difference *right now*.
- Makes your code **more organized** as your type grows.
- **Required later** for advanced features like:
  - Trait implementations
  - Conditional implementations
  - External type implementations

---

### 🧪 Example:

```rust
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_in_seconds: u32,
}

// First impl block: Associated functions (like constructor)
impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_in_seconds: u32) -> Self {
        Self {
            title,
            release_year,
            duration_in_seconds,
        }
    }
}

// Second impl block: Methods
impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!("{} ({}) - {} seconds", self.title, self.release_year, self.duration_in_seconds);
    }
}
```

#### ✅ Still works just like before:

```rust
let song = TaylorSwiftSong::new(String::from("Style"), 2014, 231);
song.display_song_info();
```

---

### 🧠 Key Takeaway:
> Multiple `impl` blocks = One merged definition.  
> Totally valid and often used in real-world Rust codebases.

