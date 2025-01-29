# Public Enums, Public Structs, and Public Fields: 

### **Key Takeaways**
1. **Using `pub` to expose module contents**  
   - Add `pub` before:
     - Constants (`pub const FLOOR_SPACE: u32 = 1000;`)
     - Enums (`pub enum ProductCategory`)
     - Structs (`pub struct Item`)
     - Functions (`pub fn talk_to_manager()`)
   - This allows these elements to be accessed outside the `inventory` module.

2. **Accessing Module Contents in `main.rs`**
   - **Constants:** `inventory::FLOOR_SPACE`
   - **Functions:** `inventory::talk_to_manager()`
   - **Enums:** `inventory::ProductCategory::Hammer`
   - **Structs:** `inventory::Item`
   
3. **Printing Debug Values**
   - Enums do not implement the `Display` trait, so use `:?` inside `println!`:
     ```rust
     println!("My favorite category of item is {:?}", favorite_category);
     ```

4. **Creating a Struct Instance (`Item`)**
   - If the struct is public (`pub struct Item`), you **can reference it** from `main.rs`.
   - However, **its fields must also be `pub`** to be accessed outside the module.
   - If `Item`’s fields are private (`name: String` without `pub`), the compiler will throw an error.

---

### **Potential Issue with `Item` Struct**
You haven’t shown the full `Item` definition yet, but if it looks like this:

```rust
pub struct Item {
    name: String,
    price: u32,
}
```
Then trying to create an instance like this:

```rust
let tall_ladder = inventory::Item {
    name: String::from("Ladder-o-matic 2000"),
    price: 500,
};
```
will **fail** because `name` and `price` are private by default.

**Fix:** Add `pub` to the struct fields:
```rust
pub struct Item {
    pub name: String,
    pub price: u32,
}
```
This will allow field initialization in `main.rs`.

---

### **Summary**
- You need `pub` on **both** the struct and its fields to fully expose it.
- If you only use `pub struct Item`, **you can reference the struct type but not instantiate it**.
- If the compiler throws an error, check if struct fields are private.

