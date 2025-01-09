### The Match keyword IV

---

### Matching Multiple Values in a Single Arm
There are two main ways to handle multiple cases in a single `match` arm:
1. **Using the Underscore (`_`) for Catch-All Matches**
2. **Using the Vertical Pipe (`|`) for Specific Multiple Matches**

---

### Example 1: Matching Multiple Variants Using `|`

Define an enum representing the stages of an online order:

```rust
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered.");
            }
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your item is being prepped for shipment.");
            }
            OnlineOrderStatus::Shipped => {
                println!("Your item is on the way!");
            }
        }
    }
}

fn main() {
    let order = OnlineOrderStatus::Packed;
    order.check(); // Output: Your item is being prepped for shipment.
}
```

---

### Example 2: Using `_` for a Catch-All Case

If you want a single arm to match all remaining cases, use `_`:

```rust
impl OnlineOrderStatus {
    fn check_with_catch_all(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered.");
            }
            _ => {
                println!("Your item is still in progress.");
            }
        }
    }
}

fn main() {
    let order = OnlineOrderStatus::Shipped;
    order.check_with_catch_all(); // Output: Your item is still in progress.
}
```

---

### Example 3: Capturing the Matched Value with a Variable

Instead of `_`, use a named variable to capture the matched value and use it in the logic:

```rust
impl OnlineOrderStatus {
    fn check_with_capture(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered.");
            }
            other_status => {
                println!("Your item is {:?}", other_status);
            }
        }
    }
}

fn main() {
    let order = OnlineOrderStatus::Shipped;
    order.check_with_capture(); // Output: Your item is Shipped
}
```

---

### Key Points:
1. **Using `|`**:
   - Combine multiple patterns in one arm.
   - Useful when the same logic applies to several cases.

2. **Using `_`**:
   - A catch-all pattern for unmatched cases.
   - Must be the last arm in the `match` block.

3. **Capturing Values**:
   - Assign a name (e.g., `other_status`) to capture the unmatched value.
   - Ensure the enum derives `Debug` or `Display` for formatting when interpolating.

4. **Debugging with `#[derive(Debug)]`**:
   - Add `#[derive(Debug)]` to the enum for `Debug` formatting in `println!`.
