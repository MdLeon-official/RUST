# Defining Struct Methods


...we can run on the struct instance.

So, to complete that thought, here's how you invoke the method:

```rust
song.display_song_info();
```

This line tells Rust:
- “Take the `song` instance (of type `TaylorSwiftSong`)”
- “Access the `display_song_info` method defined for it in the `impl` block”
- “And run it.”

---

### Full Working Example

```rust
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info(); // This takes ownership of `song`
}
```

---

### Key Concepts Recap

- `impl` is where you attach methods to your struct.
- `self` is the struct instance. It can come in 4 flavors:
  - `self` → takes ownership.
  - `mut self` → takes ownership + allows mutation.
  - `&self` → borrows immutably.
  - `&mut self` → borrows mutably.
- Capital `Self` refers to the type (like `TaylorSwiftSong`) dynamically.
- Methods are useful for organizing struct-related behavior.
