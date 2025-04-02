# Strings and Raw Strings


## String Literals
A **string** is a sequence of characters. In Rust, we declare a string using double quotes:

```rust
let greeting = "Hello, World!";
println!("{}", greeting); // Output: Hello, World!
```

This type of string is called a **string literal**, and it is known at compile time.

## Special Characters in Strings
Rust allows special characters in strings using escape sequences:

```rust
let message = "Dear Emily,\nHow have you been?";
println!("{}", message);
```
**Explanation:** `\n` creates a new line.

Other special characters:
- `\t` → Tab space
- `\"` → Double quote
- `\\` → Backslash

### Example of Special Characters:
```rust
let quote = "Juliet said, \"I love you, Romeo.\"";
println!("{}", quote);
```
**Output:**
```
Juliet said, "I love you, Romeo."
```

## Escape Sequences for File Paths
In Windows file paths, backslashes must be escaped:

```rust
let file_path = "C:\\My Documents\\new\\videos";
println!("{}", file_path);
```

## Raw Strings
Raw strings treat special characters literally:

```rust
let raw_path = r"C:\My Documents\new\videos";
println!("{}", raw_path);
```
This avoids the need for extra escaping.

### Summary:
- **String literals** are known at compile time.
- **Special characters** like `\n`, `\t`, and `\\` modify string formatting.
- **Raw strings** (`r"..."`) treat special characters as normal text.
