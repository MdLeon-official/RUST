# Or Logic with ||

1. **OR Operator (`||`)**:
   - This operator checks if **at least one** of the conditions is true.
   - If either of the conditions is true, the OR operation returns `true`.
   - If both conditions are false, the OR operation returns `false`.

### Example Scenario:
- Suppose you're building a website with premium content, and users can access it in two ways:
  1. By **paying for a subscription**.
  2. By being an **admin** (which gives them automatic access).
  
- The user can access the premium content if **either** of these conditions is true.

### Example Code:
```rust
fn main() {
    let user_has_paid_for_subscription = true;
    let user_is_admin = true;
    
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    
    println!("Can this user see the site? {}", user_can_see_premium_experience);  // true
}
```

- **OR Logic**:
  - If **either** `user_has_paid_for_subscription` **or** `user_is_admin` is true, then `user_can_see_premium_experience` will be `true`.
  - If both are false, the result will be `false`.

### Example with Both Conditions False:
```rust
fn main() {
    let user_has_paid_for_subscription = false;
    let user_is_admin = false;
    
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    
    println!("Can this user see the site? {}", user_can_see_premium_experience);  // false
}
```

### Short Circuit Evaluation:
- Rust uses **short-circuit evaluation** for the OR operator as well:
  - If the first condition is `true`, Rust doesn’t evaluate the second condition, because the result is already `true`.
  - If the first condition is `false`, Rust evaluates the second condition to determine the result.

### Combining Multiple Conditions:
- You can combine multiple conditions with both **AND** and **OR** operators.
- For example, you can check multiple independent conditions using `&&` (AND) and `||` (OR).

```rust
fn main() {
    let condition_1 = true;
    let condition_2 = false;
    let condition_3 = true;
    
    let result = condition_1 || condition_2 || condition_3;  // Evaluates to true
    println!("Final result: {}", result);
}
```

### Summary:
- The **OR operator** (`||`) returns `true` if **at least one** of the conditions is true.
- Rust uses **short-circuit evaluation**, so it stops evaluating further once a result is determined.
- You can combine multiple conditions using both **AND** and **OR** operators to model more complex logic.
