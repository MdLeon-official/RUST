# The Display Trait


- **Traits** are similar to real-world contracts. A trait defines a set of methods that a type must implement. In other words, it’s a promise that a type will support certain behaviors or actions.
  
- The **Display trait** requires a type to have a method called `format`, which allows it to be represented as a human-readable string. Rust uses this `format` method behind the scenes when we use curly brace interpolation (`{}`) in strings.

- **Types that implement Display** include integers, floats, and booleans, because they are straightforward to represent as strings (e.g., `5`, `true`, `3.14`). However, more complex types, like arrays, don't always have a clear way to represent them as readable strings, so they don't implement `Display` by default.

- **Arrays** don’t implement `Display`, which is why attempting to interpolate an array into a string using `{}` will result in a compiler error. However, there is another trait, `Debug`, that we’ll cover in the next lesson to handle this case.

### Key Points:
- A **trait** defines a contract that types can implement.
- The **Display trait** requires a type to implement the `format` method for human-readable string output.
- Not all types implement the Display trait (like arrays), and trying to interpolate a non-Display type will result in a compilation error.
