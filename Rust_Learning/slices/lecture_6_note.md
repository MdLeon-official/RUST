# String Slices as Function Parameters



1. **`String`** = Heap-allocated growable string.
2. **`&String`** = Reference to a `String`.
3. **`&str`** = String slice (usually part or all of a string).

---

#### ✅ Option 1: `&String` as Parameter

```rust
fn do_hero_stuff(hero_name: &String) {
    println!("{hero_name} saves the day");
}
```

- Only works with `&String` (reference to a `String`).
- ❌ Will **not work** with string literals (which are `&str`).

```rust
let action_hero = String::from("Arnold Schwarzenegger");
do_hero_stuff(&action_hero); // ✅ Works

let another_action_hero = "Sylvester Stallone";
do_hero_stuff(another_action_hero); // ❌ Type mismatch
```

---

#### ✅ Option 2: `&str` as Parameter (**Recommended**)

```rust
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}
```

- Works with **both** `&String` and `&str`.
- ✔️ More **versatile** and idiomatic in Rust.

```rust
let action_hero = String::from("Arnold Schwarzenegger");
do_hero_stuff(&action_hero); // ✅ &String converted to &str

let another_action_hero = "Sylvester Stallone";
do_hero_stuff(another_action_hero); // ✅ &str
```

---

### 🧪 Behind the scenes: **Deref Coercion**

Rust will **automatically convert** `&String` ➡️ `&str` (but **not the other way around**).

```rust
// Happens automatically
let s: String = String::from("Test");
let slice: &str = &s; // deref coercion
```

---

### ✅ Best Practice:

> **Use `&str` for string parameters** unless you need specific features of the `String` type.  
> It’s flexible and handles more input types.
