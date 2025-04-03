# And Logic with &&

1. **AND Operator (`&&`)**:
   - This operator checks if **both** conditions (Booleans) are **true**.
   - If both conditions are true, the result of the AND operation is `true`.
   - If either one or both conditions are false, the result will be `false`.

### Example Scenario:
- Suppose you want to catch a flight and attend an event on time. You have two independent conditions:
  1. **Purchased plane ticket** (`purchased_ticket` is `true`).
  2. **Plane is on time** (`plane_on_time` is `true`).
  
- The final condition, **making the event** (`making_event`), depends on both of these conditions being true. You can model this logic using the AND operator.

### Example Code:
```rust
fn main() {
    let purchased_ticket = true;
    let plane_on_time = true;
    
    let making_event = purchased_ticket && plane_on_time;
    
    println!("It is {} that I will arrive as expected", making_event);  // true
}
```

- **AND Logic**:
  - If both `purchased_ticket` and `plane_on_time` are `true`, `making_event` will be `true`.
  - If either of the conditions is `false`, `making_event` will be `false`.

### Short Circuit Evaluation:
- Rust uses **short-circuit evaluation** for the AND operator.
  - If the first condition is `false`, Rust doesn’t even check the second condition. This optimizes performance by avoiding unnecessary checks.
  
### Example with Different Conditions:
```rust
fn main() {
    let purchased_ticket = false;
    let plane_on_time = true;
    
    let making_event = purchased_ticket && plane_on_time;
    
    println!("It is {} that I will arrive as expected", making_event);  // false
}
```

### Summary:
- The **AND operator** (`&&`) only returns `true` if both conditions are true.
- Rust optimizes performance using short-circuit evaluation.
- This operator is useful when you need to check that multiple independent conditions are met before proceeding with an action.
