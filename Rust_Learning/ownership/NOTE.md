# Rust Ownership Basics

## 1. Ownership Basics
Ownership is a set of rules that the compiler checks for to ensure the program is free of memory errors.
- **Memory** refers to the area of your computer responsible for storing program information.
- Ideal memory management involves freeing memory when it is no longer in use.

### Key terms:
- **Allocation**: Requesting memory.
- **Deallocation**: Returning memory to the computer.

### Ownership Rules:
- Every value in Rust has a single owner at a time.
- Ownership can transfer between variables but only one owner exists at any time.
- Owners can be variables, function parameters, etc.
- Composite types (e.g., tuples, arrays) own their elements.

## 2. The Stack and the Heap
- **Stack**: Sequential, fixed-size data known at compile time. LIFO (Last In, First Out).
  - Examples: integers, floats, booleans.
  - Faster but limited to consistent-sized data.
- **Heap**: Dynamic data with unknown size at compile time.
  - Examples: user input, file contents.
  - Slower but supports growing/shrinking data.

### Key operations:
- **Stack**: Push and Pop.
- **Heap**: Allocation by a memory allocator and referencing via pointers.

## 3. Ownership and Scope
- A variable owns its value and cleans up when it goes out of scope (end of block).
- Deallocation involves popping the stack entry or cleaning heap data.

## 4. The Copy Trait
- Types with fixed sizes (e.g., integers, floats) implement the Copy trait.
- Copying creates full duplicates of these values.

## 5. String Types
- **String** (dynamic, heap-allocated): Used for text data that changes.
- **&str** (static, read-only): Known at compile time, stored in the binary executable.

## 6. Methods on String Types
- `push_str`: Appends text to a dynamic string.

## 7. Moves and Ownership
- A "move" transfers ownership from one variable to another.
- Example: Assigning a `String` variable to another transfers ownership.

## 8. Drop Function
- `drop`: Explicitly deallocates heap data.

## 9. Clone Method
- `clone`: Creates a deep copy of data (including heap data).

## 10. References and Borrowing
- Borrowing lets a value be used without transferring ownership.
- References must not outlive the referenced value.

## 11. Dereferencing Operator
- `*`: Accesses the data at the memory address pointed to by a reference.

## 12. String Types and References
- **String**: Heap-allocated text.
- **&String**: Reference to heap-allocated String.
- **str**: Static, read-only text.
- **&str**: Reference to static text.

## 13. Ownership and Function Parameters
- Parameters can take ownership or borrow.
- Immutable by default.

## 14. Mutable Parameters
- Function parameters can be mutable, allowing modifications.

## 15. Return Values
- Ownership can transfer through return values.
