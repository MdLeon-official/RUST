# Recursion


**Definition:**
Recursion occurs when a function calls itself within its own body. This technique allows solving problems by breaking them down into smaller sub-problems.

**Key Points:**
1. **Function Calls Itself**: The function invokes itself, often with a modified argument.
2. **Base Case**: The condition that stops recursion. Without it, the recursion would continue indefinitely, leading to a stack overflow.
3. **Recursive Case**: The part where the function calls itself with a modified argument to progress towards the base case.

**Mechanics of Recursion:**
- The function calls itself repeatedly, creating a nested stack of function calls.
- Each new function call awaits the completion of the one that called it, creating a "waiting" state.
- The recursion continues until the base case is met, which stops further calls.

**Example: Countdown Timer (with Recursion)**

```rust
fn countdown(seconds: i32) {
    println!("{} seconds to blastoff", seconds);
    
    // Base case: Stop recursion when seconds reach 0
    if seconds == 0 {
        println!("Blastoff!");
        return; // Stops further recursion
    }

    // Recursive case: Decrement seconds and call countdown again
    countdown(seconds - 1);
}

fn main() {
    countdown(5); // Starts countdown from 5 seconds
}
```

**How it Works:**
- The `countdown` function prints the number of seconds remaining.
- It calls itself with `seconds - 1` to count down.
- When `seconds` reaches 0, the base case is triggered, printing "Blastoff!" and ending the recursion.
- Each level of recursion completes once the deeper level finishes, "bubbling up" the results to the original function call.

**Base Case Importance:**
- Without the base case, recursion would never stop and lead to an infinite loop.
- The base case ensures the function eventually terminates, preventing an endless cycle.

**Flow Example (Countdown from 5):**
1. **First call**: Prints `5 seconds to blastoff`, calls `countdown(4)`.
2. **Second call**: Prints `4 seconds to blastoff`, calls `countdown(3)`.
3. **And so on**, until `countdown(0)` prints "Blastoff!" and stops further recursion.

**Visual Recursion Flow:**
- Multiple function calls stack on top of each other, each waiting for the previous one to complete.
- Once the base case is reached, the functions "unwind" in reverse order, returning control to the previous invocation.

### Key Takeaways:
- **Recursion**: A function calls itself with updated parameters.
- **Base Case**: Prevents infinite recursion and halts further calls.
- **Recursive Case**: The step where the function calls itself with modified parameters.
