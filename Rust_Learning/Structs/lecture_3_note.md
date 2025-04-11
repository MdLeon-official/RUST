# Access Struct Fields


#### ✅ Accessing Fields  
To **read a field** from a struct:
```rust
mocha.name  // struct_name.field_name
```

You **can’t use this directly inside `{}`** in `println!`, but you can pass it like:
```rust
println!("{}", mocha.name);
```

You can also print multiple fields:
```rust
println!("My {} cost {}. It is {} that it was hot.",
         mocha.name, mocha.price, mocha.is_hot);
```

#### 🛠 Ownership Rules with Structs
- A **struct owns its fields**, and **each field owns its value**.
- If a field (like a `String`) **doesn’t implement `Copy`**, using it in assignment **moves ownership**:
```rust
let favorite = mocha.name; // 👈 ownership moves!
println!("{}", mocha.name); // ❌ error: value moved
```
- Types like `f64`, `bool` **do get copied**, so this is fine:
```rust
let price = mocha.price;  // ✅ copied, no error
```

#### 🧠 Remembering Tip:
Think of your struct like a **treasure chest** 🧰:
- The chest (`mocha`) holds keys (`name`, `price`, `is_hot`)
- If you **take** the "name" key (`String`) out and give it to someone (`let favorite = mocha.name`), it's **gone from the chest**! Can't use it again!
