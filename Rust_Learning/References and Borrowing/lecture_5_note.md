# Dangling References


A **dangling reference** is a reference (like a pointer) to memory that **has been freed or deallocated**.

> Imagine giving someone the address of a house that has already been demolished. They go there and... nothing exists.

---

### 🛡️ **How Rust Prevents Dangling References**
Rust uses the **borrow checker** to **ensure no reference outlives the value it refers to**.

If a function tries to return a reference to a value that will be **dropped** when the function ends, Rust **won’t compile** the code.

---

### ❌ Example That Causes Error
```rust
fn create_city() -> &String {
    let city = String::from("New York");
    &city // ❌ Error: returning reference to local variable
}
```

Here, `city` is dropped when the function ends. Returning `&city` creates a **dangling reference**.

Error:
```
this function's return type contains a borrowed value, 
but there is no value for it to be borrowed from
```

---

### ✅ Correct Version
Return the **owned value** instead of a reference:

```rust
fn create_city() -> String {
    String::from("New York")
}
```

This way, ownership of the String is moved out of the function, and there’s **no reference** involved.

---

### 📌 Rules to Remember
1. A reference **must not outlive** the data it points to.
2. You **can’t return a reference** to a local variable from a function.
3. Rust enforces these rules **at compile time**, so your code is safe from memory errors.

---

quick analogy
> 🧠 *A reference is like a borrowed library book. If the library burns down (scope ends), the book (value) is gone. You can’t keep borrowing it.*

