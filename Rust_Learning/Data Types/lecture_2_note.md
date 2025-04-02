# Integers


```rust
// Declaring an 8-bit signed integer
let eight_bit: i8 = -112;

// Declaring an 8-bit unsigned integer
let eight_bit_unsigned: u8 = 255; // Valid range: 0 to 255

// Declaring a 16-bit signed integer
let sixteen_bit_signed: i16 = -32_500; // Valid range: -32,768 to 32,767

// Declaring a 16-bit unsigned integer
let sixteen_bit_unsigned: u16 = 64_000; // Valid range: 0 to 65,535

// Declaring a 32-bit signed integer
let thirty_two_bit_signed: i32 = -2_147_483_648; // Default integer type in Rust

// Declaring a 32-bit unsigned integer
let thirty_two_bit_unsigned: u32 = 4_294_967_295; // Valid range: 0 to 4,294,967,295

// Alternate syntax: Type annotation after the value
let some_value = 20i8;  // Equivalent to let some_value: i8 = 20;
let another_value = 50u16;  // Equivalent to let another_value: u16 = 50;
```

