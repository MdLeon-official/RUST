# The Dereference Operator


#### 📌 What is an operator?
An operator is a symbol that does something to a value.  
Example: `+` adds numbers.

#### ⭐ Dereference Operator: `*`
- Same symbol as multiplication (`Shift + 8`), but **different meaning** in this context.
- It **follows a reference (address)** to get the actual **value stored** at that address.

#### 🧪 Example:
```rust
let my_stack_value = 2;
let my_integer_reference = &my_stack_value;
println!("{}", *my_integer_reference); // outputs: 2
```
🔍 `*my_integer_reference` says:  
"Go to the memory address stored in this reference, and get the value there."

#### 🎯 Only Works on References
You **cannot** do `*my_stack_value` because it's not a reference. It's a real value, not an address.

#### 🧵 Same with Heap Values:
```rust
let my_heap_value = String::from("Toyota");
let my_heap_reference = &my_heap_value;
println!("{}", *my_heap_reference); // outputs: Toyota
```

---

### 💡 Behind the Scenes

If you just do:
```rust
println!("{}", my_integer_reference);
```
You **still get `2`** (the value), not the address. Why?

Rust auto-dereferences it because:
- Rust implements the **Display trait** for references.
- The Display trait makes sure the **value** at the reference is shown, not the address.

So:
- `println!("{}", *ref)` ✅
- `println!("{}", ref)` ✅ (because Rust auto-dereferences when printing)

---

### ✅ Summary
| Action                     | Output         | Notes                                           |
|---------------------------|----------------|-------------------------------------------------|
| `*reference`              | Actual value   | Manual dereference                              |
| `reference` in `println!` | Actual value   | Rust auto-dereferences because of Display trait |
| Value (not a reference)   | Direct value   | Cannot use `*` on it                            |
