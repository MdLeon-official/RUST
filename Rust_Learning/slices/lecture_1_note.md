

# Intro to Slices

#### 📌 What is a Slice?

- A **slice** is a *reference* to a **part of a collection**, like a part of a string or an array.
- It **borrows** data (doesn’t take ownership).
- Think of it like borrowing **one slice of pizza** 🍕 instead of taking the whole pizza box.

#### 🏡 Real-Life Example:
Imagine a **house**:
- A **normal reference** borrows the whole house.
- A **slice** borrows **just a room or a floor** — a *part* of the house.

#### 🔍 Types of Slices:
- `&str` → a **string slice** (part of a string)
- `&[T]` → an **array slice** (part of an array)

#### ⚠️ Tip:
Even if a slice refers to the **whole collection**, it’s *still* called a slice.
> 📌 Slice = "a part" (but that part can be small **or the full thing**)

#### Remember:
> ✅ Slice = Reference to a part  
> ❌ Slice ≠ Ownership  
> 🍕 Think pizza, not pie chart!
