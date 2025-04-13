# Associated Functions


#### 📌 What are they?
- Functions defined **on a type**, not on an instance.
- They **do not use `self`** in the parameter list.
- Called with the syntax: `Type::function_name()`

#### 🗂️ Think of it as a *namespace*:
- The type (like a struct) **acts as a folder**.
- Associated functions **live inside that folder**.
- Accessed using **double colons (`::`)**, e.g., `String::from("Hello")`

---

### 🏗️ Common Use Case: Constructors

#### 📐 Convention:
- Define a `new()` function as a **constructor**
- Returns an instance of the type

#### 🧱 Example:

```rust
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_in_seconds: u32,
}

impl TaylorSwiftSong {
    // Associated function (constructor)
    fn new(title: String, release_year: u32, duration_in_seconds: u32) -> Self {
        Self {
            title,
            release_year,
            duration_in_seconds,
        }
    }

    // Method (uses self)
    fn display_song_info(&self) {
        println!("{} ({}) - {} seconds", self.title, self.release_year, self.duration_in_seconds);
    }
}
```

#### 🧪 Usage:

```rust
let blank_space = TaylorSwiftSong::new(
    String::from("Blank Space"),
    2014,
    231,
);

blank_space.display_song_info();
```

---

### ✅ Key Differences: Associated Function vs Method

|                     | **Associated Function**      | **Method**                      |
|---------------------|------------------------------|----------------------------------|
| Uses `self`?        | ❌ No                         | ✅ Yes                           |
| Called on?          | The **type** itself           | An **instance** of the type      |
| Common use?         | Constructors like `new()`     | Behavior like `.display_info()`  |
| Syntax              | `Type::function()`            | `instance.method()`              |
