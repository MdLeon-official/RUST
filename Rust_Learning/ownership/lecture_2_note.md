# The Stack and Heap


### 🪜 Stack
- Fast and **ordered** like a stack of lunch trays 🍽️ (Last In, First Out - LIFO).
- Only stores **fixed-size** data that’s known at **compile time**.
  - Example: `i32`, `bool`, `char`, small arrays.
- When a function ends, variables on the stack go **out of scope** and get **popped off** (deleted) automatically.

🧠 Analogy:  
Think of each variable like a lunch tray:
- First tray (value) at the bottom → added first.
- Last tray on top → removed first.

✅ Stack is:
- Fast for **writing (push)** and **reading (pop)**.
- Simple and **efficient**, but limited to **predictable-sized data**.

---

### 🏗️ Heap
- Like a **warehouse or parking lot**.
- Stores **dynamic** data → size **unknown at compile time**.
  - Examples: User input, file content.
- Requires a **memory allocator** to find space in heap.

📦 How it works:
- You ask for space → allocator finds it → gives you a **reference (address)** to it.
- That reference is stored on the **stack**, but the actual data lives in the **heap**.

🏡 Real-World Analogy:
- House = Data in heap.
- Address = Pointer/reference stored on stack.

📋 Example:
```rust
let x = String::from("hello"); // "hello" is stored on heap
```

- The string data lives in the **heap**.
- The **pointer to it** (its address) is stored on the **stack**.

---

### ⚡ Stack vs Heap Summary

| Feature        | Stack                         | Heap                            |
|----------------|-------------------------------|----------------------------------|
| Speed          | Very fast ⚡                   | Slower 🐢                        |
| Data Type      | Fixed-size (known at compile) | Dynamic-size (runtime decided)  |
| Allocation     | Easy: push/pop                | Hard: search space via allocator|
| Example        | `i32`, `bool`, arrays         | `String`, user input, file data |
| Access         | Direct                        | Via reference (pointer)         |

---

### 🧹 Ownership in Rust
- Rust uses **ownership** to manage **heap memory**.
- When a variable goes out of scope, Rust **automatically deallocates** its heap memory.
- This avoids memory leaks and duplicate heap data without needing a garbage collector.

---

✅ **Remember**:  
- Use **stack** when size is fixed and known.  
- Use **heap** when data is big or size is unknown (like user input).  
- Stack is faster, but heap is more flexible.
