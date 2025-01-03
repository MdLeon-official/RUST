# Notes on Structs in Rust

## L1: Struct Overview:
A struct is a container for related pieces of data, similar to an object in object-oriented programming. It allows grouping multiple pieces of data together in a single, meaningful unit.

## Comparison with Tuples:
While tuples can store multiple pieces of data, the order of elements in a tuple can be arbitrary, and there is no context about what each value represents. Structs, on the other hand, use named fields to provide context and allow easier access to individual pieces of data.

## Types of Structs:
- **Named Field Structs**: The most common type, where each field has a name and a corresponding value.
- **Tuple-Like Structs**: Similar to tuples but with named fields, offering more context and clarity.
- **Unit-Like Structs**: Structs without fields, often used as markers or for unit tests.

## Why Use Structs:
Structs are useful for representing complex data types with multiple fields that can vary in type. They provide a clearer, more readable way of grouping related data.

## Struct Definition:
A struct is a blueprint for creating instances of a complex data type. It doesn't store concrete data directly but specifies what kind of data it will hold. Fields are defined with names and types.

## Naming Conventions:
- **Struct Name**: Typically written in PascalCase (e.g., Coffee).
- **Field Names**: Typically written in snake_case (e.g., is_hot, price).

## Using Structs:
Once a struct is defined, instances of that struct can be created by providing concrete values for its fields.
Fields are accessed using dot notation (e.g., coffee.name, coffee.price), making it easy to work with the data inside the struct.

## Conclusion:
Structs allow for better organization of related data and provide clarity in accessing and modifying that data.
Named field structs are particularly useful when modeling real-world objects or concepts in your code.


---
---


# L2: Creating an Instance of a Struct

## Blueprint vs Instance
A struct defines a blueprint for data, and when we create a concrete value using that blueprint, we call it an instance of the struct.

## Defining an Instance:
- Start by defining a variable to hold the instance (e.g., `mocha`).
- Assign it a value from the struct type by writing the struct name followed by curly braces `{}`.
- Inside the curly braces, provide values for each field in the struct, matching the field names defined in the struct.

## Field Order:
The order of fields does not matter when creating an instance because fields are accessed by their names, not by their position.

## Field Values:
Each field in the struct must be assigned a value that matches the expected type from the struct definition. Rust checks the type at compile-time, and any type mismatch will result in a compilation error.

## Error Handling:
- If the wrong type is provided for a field (e.g., providing an integer when a Boolean is expected), Rust will generate a "mismatched types" error.
- If any field is omitted during initialization, Rust will show an error indicating which field is missing.

## Summary:
When creating an instance of a struct, ensure all fields are provided with valid values of the correct types, and remember that the order of fields doesn't affect the instance creation.

---
---

# L3: Accessing and Understanding Ownership in Structs

## 1. Accessing Struct Fields:
- To access the value of a struct field, reference the struct variable, followed by a dot (`.`) and the field's name.
- Example: `mocha.name` allows us to access the `name` field of the `mocha` struct.

## 2. Using `println!` Macro for Output:
- When printing struct fields, the struct field is provided as an argument inside the `println!` macro.
- Field names can be accessed with the dot notation, e.g., `mocha.name`, `mocha.price`, and `mocha.is_hot`.

## 3. Ownership in Structs:
- A struct owns its fields, and each field owns its value.
- When a struct goes out of scope, it and its fields are cleaned up, respecting ownership rules.

## 4. Hierarchical Ownership:
- Ownership is hierarchical. For example, the `mocha` variable owns the `Coffee` struct, and the struct in turn owns its fields (e.g., `name`, `price`, `is_hot`), which in turn own their respective values.
- This leads to a cascade of memory cleanup when variables go out of scope.

## 5. Movement of Ownership:
- Types that do not implement the `Copy` trait, like `String`, transfer ownership when assigned to another variable.
- Example: Assigning `mocha.name` (which is a `String`) to `favorite_coffee` moves ownership from the struct field to the new variable.
- Once ownership is moved, the original field is no longer the owner, leading to borrowing issues if accessed later.

## 6. Errors Due to Ownership Movement:
- After ownership has been moved, trying to access the original owner (e.g., `mocha.name` after it has been assigned to `favorite_coffee`) will result in a compiler error because the original owner no longer exists.

## 7. Copyable Types vs Non-Copyable Types:
- Types stored on the stack (e.g., `f64`, `bool`) can be copied when assigned, while heap-allocated types like `String` transfer ownership rather than being copied.

## 8. Conclusion:
- Ownership in Rust is critical to managing memory and preventing errors.
- The compiler helps catch issues related to ownership, ensuring safe memory management.

---
---


### L4: Overwriting Struct Field Values

1. **Mutability of Struct Instances:**
   - To modify the fields of a struct instance, the struct instance itself must be mutable.
   - Use the `mut` keyword when declaring the variable to make it mutable (e.g., `let mut beverage`).
   - By default, variables in Rust are immutable, so you must explicitly allow changes by marking the instance as mutable.

2. **Immutability vs Mutability:**
   - The definition of a struct is not mutable or immutable; it's just a template.
   - The mutability applies only to the specific instance of the struct (e.g., `beverage`).
   - The entire struct instance must be either mutable or immutable. You cannot have partial mutability, where only some fields of the struct are mutable.

3. **Overwriting Struct Field Values:**
   - To overwrite a field's value, reference the struct instance, followed by a dot (`.`), then the field name.
   - Use the assignment operator (`=`) to assign a new value to the field.
   - When a field’s value is overwritten, the field takes ownership of the new value, and Rust will automatically clean up the previous value if needed.

4. **Example Process:**
   - After making the struct instance mutable, you can update each field one by one using the dot notation.
   - Each field can be updated individually (e.g., `beverage.name = "Caramel Macchiato"`).
   - All fields of the mutable struct can be updated, or just the specific fields you want to change.

5. **Output and Reflection of Changes:**
   - After updating the fields, you can print the values to verify the changes reflect the new values.
   - Remember to update the variable names if you change the struct instance name (e.g., from `mocha` to `beverage`).

6. **Key Takeaway:**
   - The entire struct must be mutable if any field values are to be overwritten.


---
---


# L5: Creating and Returning Struct Instances in Rust

## 1. Function Definition and Struct Placement
- Define a new function (`make_coffee`) that creates and returns a `Coffee` instance.
- Move the `Coffee` struct definition outside the `main` function to make it accessible throughout the file.
- A struct defined at the file (or module) level has a more global scope, allowing it to be used in any function within the file.

## 2. Function Parameters and Return Type
- The `make_coffee` function accepts three parameters:
  - `name` (String)
  - `price` (f64)
  - `is_hot` (bool)
- Specify the return type using the arrow syntax (`-> Coffee`).

## 3. Struct as a Return Value
- Structs are valid types in Rust and can be returned by functions.
- Use Rust’s implicit return syntax to create and return a new `Coffee` instance within the function body.

## 4. Dynamic Field Assignment
- Assign function parameters to struct fields dynamically.
- The syntax `field_name: parameter_name` binds the passed parameter values to their respective fields.

## 5. Ownership Transfer
- Ownership moves multiple times during the process:
  - A String created on the heap is initially owned by the variable (e.g., `name`).
  - Ownership transfers to the parameter when passed to the function.
  - Inside the function, ownership transfers again from the parameter to the struct field.
- When the struct is returned, the caller function's variable becomes the owner of the struct and its fields.

## 6. Using and Verifying Struct Instances
- Assign the returned struct to a variable (e.g., `coffee`) in `main`.
- Rust infers the variable type based on the return type of the function.
- Optionally, you can explicitly annotate the variable type.
- Use struct fields in output or further operations (e.g., `coffee.name`, `coffee.price`).

## 7. Exploring String Ownership
- If a String is declared separately (e.g., `name`), passing it to the function without a reference transfers ownership.
- After assignment to the struct field, the function’s parameter loses ownership.
- The struct instance becomes the final owner of the String through its field.

## 8. Struct Ownership Hierarchy
- The variable owning the struct owns its fields.
- Struct fields, in turn, own their respective values (e.g., a field owning a String).

---
---

# L6: Simplified Struct Creation Syntax

- Rust allows a shorthand syntax when creating structs if the parameter or variable names match the struct field names.
- This eliminates the need for the `field: field` syntax and simply uses the field name.

## Key Points:
- When the struct field names and parameter/variable names match, Rust automatically connects them.
- This reduces the amount of code written and improves readability.

## Example:
If you have a struct with fields `name`, `price`, and `is_hot`, and parameters with the same names, you can instantiate the struct like this:
```rust
Coffee { name, price, is_hot }
```

## Why It’s Useful:
- Simplifies the syntax.
- Keeps naming consistent between function parameters, variables, and struct fields.

## Important Notes:
- The shorthand only works if the parameter/variable name matches the struct field name.
- If they differ (e.g., `my_price` instead of `price`), Rust will not make the connection automatically.


---
---

# L7: Struct Update Syntax

## Creating a New Struct from an Existing One:
- You can create a new struct instance using values from an existing instance.
- To manually assign values, reference the fields from the existing struct for fields that should remain unchanged, while assigning new values to other fields.

## Struct Update Syntax:
- Rust provides a shorthand called struct update syntax for copying fields from another struct.
- The syntax involves using two dots (`..`) followed by the struct to copy fields from.
- This syntax reduces manual effort, especially for structs with many fields.

### Key Rules for Struct Update Syntax:
1. Fields explicitly defined before the `..` will not be copied from the source struct.
2. The `..` syntax must always come last in the list of fields.
3. Only fields implementing the `Copy` trait (like numbers or booleans) can be directly copied without issues.

## Ownership Concerns with Non-`Copy` Fields:
- Fields that do not implement the `Copy` trait, like `String`, result in ownership transfer when copied using the `..` syntax.
- Ownership transfer invalidates the original field in the source struct, making it unusable.

### Avoiding Ownership Issues:
- For fields like `String`, cloning is necessary to create a duplicate, allowing both structs to retain independent ownership of the data.
- Cloning ensures that the original struct remains intact and functional.

## Practical Considerations:
- Struct update syntax is equivalent to assignment, and the compiler treats it similarly.
- Be cautious with ownership rules in Rust to avoid unexpected errors.
- Using struct update syntax can make your code cleaner and more concise, but always consider cloning for non-`Copy` fields to prevent ownership problems.

---
---

# L8: Passing a Struct to Functions in Rust - Notes

## Introduction
- Structs serve as excellent function arguments because they bundle data into a single composite value, making function definitions simpler by reducing the number of parameters.
- Functions can receive struct instances in four different ways, each with its own characteristics and implications for ownership, mutability, and references.

## 1. Passing an Immutable Struct by Value
- The struct is passed as a parameter, and ownership is transferred from the caller to the function.
- Once ownership is transferred, the original variable in the caller becomes invalid and cannot be used further.
- The function can read the fields of the struct but cannot modify them, as the parameter is immutable by default.
- The struct instance is cleaned up when the function scope ends unless explicitly returned to the caller.

### Key Points:
- Ownership is transferred.
- The struct cannot be mutated in the function.
- The original variable becomes invalid.

## 2. Passing a Mutable Struct by Value
- Ownership of the struct is still transferred, but the function is allowed to modify its fields because the parameter is declared as mutable.
- Similar to the immutable case, the original variable becomes invalid after the function call.
- The struct is deallocated at the end of the function unless explicitly returned to the caller.

### Key Points:
- Ownership is transferred.
- Fields of the struct can be modified in the function.
- The original variable is invalidated unless ownership is returned.

## 3. Passing an Immutable Reference to a Struct
- Instead of passing the actual struct, a reference to the struct is passed, allowing the function to access the original struct without taking ownership.
- The reference provides read-only access to the fields.
- Since ownership is not transferred, the original variable remains valid and can be used after the function call.
- Rust automatically dereferences references when using dot syntax to access fields, providing a seamless experience.

### Key Points:
- Ownership is not transferred.
- The function can read fields but cannot modify them.
- The original variable remains valid.

## 4. Passing a Mutable Reference to a Struct
- A mutable reference allows the function to modify the fields of the struct without taking ownership.
- The original variable remains the owner, but it must be declared mutable to allow modifications through the reference.
- Passing a mutable reference avoids duplicating the struct or transferring ownership, making it an efficient design choice.
- The reference itself is cleaned up at the end of the function scope, but the original struct remains intact.

### Key Points:
- Ownership is not transferred.
- The function can read and modify fields of the struct.
- The original variable remains valid and retains ownership.

## Key Advantages of References
- **No Ownership Transfer:** References allow the original owner to retain control over the struct.
- **Avoids Duplication:** Passing references avoids creating unnecessary duplicates of the struct.
- **Simplified Design:** Functions can modify the struct without requiring it to be returned after each call.

## Choosing the Right Approach
- Use **immutable references** when the function only needs to read data from the struct.
- Use **mutable references** when the function needs to modify the struct but ownership should remain with the caller.
- Use **passing by value** when ownership needs to be transferred, either immutably or mutably.
- Avoid passing ownership back and forth across multiple functions to maintain a clean and efficient design.

## Conclusion
The choice between passing by value or by reference, and whether to allow mutability, depends on the specific needs of the function and the desired ownership behavior. Using references is often the preferred approach as it avoids unnecessary ownership complications and keeps the design straightforward.

---
---

### L9: Traits, Display, Debug, and Struct Debug Implementation

#### **Traits Overview**
- **Traits** define a contract that mandates a type must support one or more methods.
- A type that implements a trait "promises" to provide certain functionality.
- Types can choose to implement whichever traits they wish.

#### **Display and Debug Traits**
- **Display Trait**:
  - Represents a type as a human-friendly, readable string.
  - Used in string interpolation with `{}`.
  - Not all types implement the Display trait (e.g., arrays).

- **Debug Trait**:
  - Represents a type as a technical string, primarily for developers.
  - Used with `{:?}` for standard Debug representation.
  - Used with `{:#?}` for pretty-printed Debug representation.

#### **Example: Arrays and Traits**
- Arrays do not implement the **Display Trait**:
  - Using `{}` with an array causes a compile error.
  - Error: "Array of string slices does not implement the Display trait."

- Arrays implement the **Debug Trait**:
  - Using `{:?}` or `{:#?}` with an array prints its elements.
    - `{:?}`: Standard Debug representation.
    - `{:#?}`: Pretty-printed Debug representation.

#### **Structs and Traits**
- **Default Behavior**:
  - Structs do not implement the **Display Trait** or **Debug Trait** by default.
  - Attempting to use `{}`, `{:?}`, or `{:#?}` results in compile-time errors.

- **Deriving Debug Implementation**:
  - Rust provides an easy way to derive a Debug implementation for structs.
  - The derived Debug implementation includes:
    - Struct name.
    - Curly braces with fields and their corresponding values.

#### **Derive Attribute for Debug**
- **Attributes**:
  - Directives or metadata for the compiler written above a construct.
  - Customize how the compiler processes the construct.

- **Syntax for Deriving Debug**:
  ```rust
  #[derive(Debug)]
  struct Coffee {
      price: f64,
      name: String,
      is_hot: bool,
  }
  ```
  - `#[derive(Debug)]`: Automatically generates a Debug implementation.

- **Usage**:
  - `{:?}`: Standard Debug output.
  - `{:#?}`: Pretty-printed Debug output.

#### **Example: Deriving Debug for Struct**
```rust
#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    // Using Debug trait
    println!("{:?}", mocha);    // Standard Debug representation
    println!("{:#?}", mocha);  // Pretty-printed Debug representation
}
```
- Output:
  ```
  Coffee { name: "Mocha", price: 4.99, is_hot: true }

  Coffee {
      name: "Mocha",
      price: 4.99,
      is_hot: true,
  }
  ```

#### **Customizing Debug Implementations**
- While deriving Debug is a convenient default, you can manually define Debug implementations.
- Custom implementations allow full control over how a type represents itself for debugging.
- This will be discussed in later lessons.

#### **Recap**
- Traits like Display and Debug allow types to be represented as strings.
- The `#[derive(Debug)]` attribute provides a quick way to enable Debug for structs.
- Use `{:?}` and `{:#?}` for standard and pretty-printed Debug outputs, respectively.
- Derived Debug outputs are practical for debugging but can be customized when needed.


