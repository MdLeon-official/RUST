# Deriving Debug Trait for Struct


#### 🌟 What is a Trait?
- A **trait** is like a *contract* or *promise* that a type will support certain functionality (i.e., methods).
- Types in Rust can **opt-in** to implementing traits.

---

### 🖋️ `Display` Trait
- For **human-friendly output**.
- Used when you use `{}` inside `println!`.
- **Arrays and custom structs don't implement `Display` by default.**

```rust
println!("{}", my_struct); // Needs Display trait
```

---

### 🛠️ `Debug` Trait
- For **developer-friendly, technical output**.
- Used with `{:?}` or `{:#?}` inside `println!`.

```rust
println!("{:?}", my_struct);   // Compact debug print
println!("{:#?}", my_struct);  // Pretty-printed debug print
```

- Arrays *do* implement `Debug` by default.
- Custom structs *don't*.

---

### 🧩 Deriving Traits (Like `Debug`)
To automatically make your struct printable in debug format:

```rust
#[derive(Debug)]
struct Coffee {
    name: String,
    size: String,
}
```

Now you can do:

```rust
let mocha = Coffee {
    name: "Mocha".to_string(),
    size: "Medium".to_string(),
};

println!("{:?}", mocha);    // Debug output
println!("{:#?}", mocha);   // Pretty Debug output
```

---

### 🏷️ Attributes in Rust
- Start with `#` and `[]`.
- They give **extra instructions** to the compiler.
- Example: `#[allow(unused_variables)]`
- Used for `derive` too: `#[derive(Debug)]`

---

### 🧠 Remembering Tip:
> **`Display` is for people**,  
> **`Debug` is for devs.**
