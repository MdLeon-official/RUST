# Intro to data types:
- Rust is a statically typed language, meaning the compiler must know the type of every variable at compile time.
- The compiler can infer types but also allows explicit annotation.

## Categories of Data Types
1. **Scalar Types** (Hold single values)
   - Integer types
   - Floating-point types
   - Boolean
   - Character

2. **Compound Types** (Hold multiple values)
   - Tuples
   - Arrays

## Integer Types
- **Signed integers (`i`)**: Support both positive and negative values.
- **Unsigned integers (`u`)**: Support only non-negative values (0 and positive).

### Integer Bit Sizes
| Type  | Size (bits) | Range (Signed) | Range (Unsigned) |
|-------|------------|---------------|------------------|
| i8    | 8          | -128 to 127   | 0 to 255        |
| i16   | 16         | -32,768 to 32,767 | 0 to 65,535 |
| i32   | 32         | -2,147,483,648 to 2,147,483,647 | 0 to 4,294,967,295 |
| i64   | 64         | -9 quintillion to 9 quintillion | 0 to 18 quintillion |
| i128  | 128        | Extremely large range | Extremely large range |

- **Signed integers require storage for the sign bit**, reducing their max positive value.
- **Unsigned integers have a larger positive range** since they don't store a sign.

## Floating-Point Types
- `f32`: 32-bit, ~6-9 digits of precision.
- `f64`: 64-bit, ~15-17 digits of precision (default in Rust).

## Memory Measurement Units
- **Bit**: Smallest unit (0 or 1).
- **Byte**: 8 bits.
- **Kilobyte (KB)**: 1024 bytes.
- **Megabyte (MB)**: 1024 KB.
- **Gigabyte (GB)**: 1024 MB.

## Key Takeaways
- Use **signed integers** (`i8`, `i16`, etc.) when negative values are required.
- Use **unsigned integers** (`u8`, `u16`, etc.) when only positive values are needed.
- Rust defaults to `i32` for integers and `f64` for floats.
- **Avoid premature memory optimization**; focus on code clarity first.
