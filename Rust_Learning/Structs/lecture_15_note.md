# Builder Pattern


### 🧠 What is it?
The **Builder Pattern** is a design pattern — a smart way to write code that makes building or modifying objects clean and easy.

Imagine you're ordering a custom computer. You start with a basic one and tell the shop:
- “Change the CPU”
- “Add more RAM”
- “Upgrade the hard drive”

Each time, the shop returns the same computer with your change. That’s what the Builder Pattern does in code.

### 🧩 Key Idea:
- Each method **modifies the object** and then **returns the object again** (or a reference to it).
- This allows **method chaining** — calling methods one after another.

### 📦 Example:
```rust
let mut computer = Computer::new("M3 Max".to_string(), 64, 2);
computer
    .upgrade_cpu("M4 Max".to_string())
    .upgrade_memory(128)
    .upgrade_hard_drive_capacity(4);
```

Without builder pattern, you'd write:
```rust
computer.upgrade_cpu("M4 Max".to_string());
computer.upgrade_memory(128);
computer.upgrade_hard_drive_capacity(4);
```

### 🔧 How to Build It:
- Each method takes `&mut self` (so you can mutate).
- Return `&mut Self` at the end.
```rust
fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
    self.cpu = new_cpu;
    self
}
```

Do the same for `upgrade_memory` and `upgrade_hard_drive_capacity`.

### ✅ Why Use It?
- Cleaner and **easier to read**.
- Saves repeating variable name (`computer`) every time.
- Looks like you’re **building up** the object step by step.

### 🧠 Remember Tip:
> “**Upgrade, Return, Chain**”  
> Every method **upgrades** a part, **returns** the object, so you can **chain** the next call.
