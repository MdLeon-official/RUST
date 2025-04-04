
# Intro to Functions

#### 🧠 What is a function?
- A **function** is a reusable sequence of instructions (logic).
- Declared with the `fn` keyword.
- Helps **organize code**, promote **reusability**, and **avoid repetition**.

#### 🧷 Function Syntax:
```rust
fn function_name() {
    // function body
}
```
- Use `snake_case` for function names (`my_function`, `bake_pizza`).
- Logic goes inside `{}`.
- To **call/invoke** a function: `function_name();`

#### 🧠 Main Function:
- Entry point of the program.
- Rust automatically runs `fn main()` once.
- You can call other functions **inside main**.

#### 🧪 Example:
```rust
fn open_store() {
    println!("Opening my pizza store");
}

fn bake_pizza() {
    println!("Baking a pizza one more time");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}

fn main() {
    open_store();
    bake_pizza();
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
}
```

#### 🔁 Why use functions?
- Reusability: Avoid repeating code.
- Organization: Break complex tasks into simpler steps.
- Readability: Code becomes easier to understand and maintain.
