# Calling Methods from Other Methods


### Invoking Methods Within Other Methods:
- You can invoke other methods from within a method body, which helps keep your code modular and reusable.
- Small methods that perform individual tasks can be combined to build more complex behaviors in a larger method.
- This approach makes your code easier to change because each operation is decoupled from others.

### Example: `years_since_release` Method
- **Goal:** Calculate the number of years since a song's release.
- **Method Definition:** The `years_since_release` method takes an immutable reference to the struct (`&self`) and returns a `u32` (the number of years).
  
```rust
fn years_since_release(&self) -> u32 {
    2024 - self.release_year
}
```

- **Using `years_since_release` within `display_song_info`:** The `display_song_info` method uses the `years_since_release` method to display the number of years since the song's release.

```rust
fn display_song_info(&self) {
    println!("Title: {}", self.title);
    println!("Years since Release: {}", self.years_since_release());
}
```

- In the main function, you invoke `display_song_info` on an instance of the struct (`blank_space`), which calls the `years_since_release` method internally and prints the result.

### Key Takeaways:
- **Small Helper Methods:** Break down tasks into small, reusable methods that do one thing well. This helps in making your code modular and maintainable.
- **Method Invocation:** You can invoke methods from other methods using `self` or `&self`. Even if a method has additional parameters, you can still invoke it easily as long as you pass the required arguments.
- **Reusability:** By isolating specific logic into separate methods (like `years_since_release`), you make it easier to reuse that logic in different parts of your program.

