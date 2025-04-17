# References and Borrowing



### 🧠 Borrowing = Using Without Owning
- Instead of copying, we can **borrow** a value.
- **Borrowing = using the value temporarily without taking ownership**.
- Example: Borrowing your friend’s car. You don’t own it; you just use it for a while and return it.

---

### 🔗 References
- A **reference** is a **memory address** pointing to the actual data.
- Think of the value as a **house**, and the reference is the **address written on a paper**.
- The reference (address) is stored using the `&` (ampersand) symbol.

```rust
let my_stack_value = 2;                 // Lives on stack
let my_integer_reference = &my_stack_value; // &i32 => reference to an i32

let my_heap_value = String::from("Toyota");   // Lives on heap
let my_heap_reference = &my_heap_value;       // &String => reference to a String
```

---

### 🔁 Reference ≠ Original
- `String` ≠ `&String`
- They are different types!
- A reference is a **cheap way** to use a value without copying it.

---

### 📚 Stack vs Heap
- Stack values (like integers) are cheap to copy → we usually just clone them.
- Heap values (like `String`) are **better borrowed** to avoid expensive clones.

---

### 🧷 Reference = Safe Pointer
- A **reference is a pointer** with **guarantees**:
  - Always points to a **valid** value.
  - Can never **outlive the value** it points to.
- Rust ensures safety:
  - No more “pointer to a deleted value” (common bug in C/C++).

```rust
// Good: my_heap_reference ends before my_heap_value
```

🧠 Rule:  
**“A reference must never outlive the referent.”**  
- `referent` = the actual value  
- `reference` = the pointer (ampersand thing)

---

### 🏁 Cleanup Order
At the end of a scope:
- Reference variables are cleaned up first.
- Then the actual values are dropped (owners deallocate them).

---

### 💡 Quick Summary
| Term           | Meaning                                          |
|----------------|--------------------------------------------------|
| `&value`       | Reference (borrow the value)                     |
| Reference      | A memory address pointing to a value             |
| Borrowing      | Using a value temporarily without owning it      |
| Stack vs Heap  | Copy stack values, borrow heap values            |
| Safety         | References always point to valid memory in Rust  |
