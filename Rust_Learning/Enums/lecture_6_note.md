### Nesting enums in enums

```rust
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

fn main() {
    // Creating instances of RestaurantItem
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;

    // Displaying the instances using Debug formatting
    println!("Lunch was: {:?}.", lunch);
    println!("Dinner was: {:?}.", dinner);
    println!("Nobody ate: {:?}.", abandoned_meal);
}
```

### Explanation of Key Concepts:
1. **Enums as Data Types**:
   - Enums can represent a set of related variants.
   - Variants can store data, either inline (tuple variants) or named (struct variants).

2. **Tuple vs. Struct Variants**:
   - Tuple Variants: Use parentheses to encapsulate associated data in a specific order.
     ```rust
     enum Example {
         TupleVariant(i32, String),
     }
     ```
   - Struct Variants: Use curly braces with named fields for associated data.
     ```rust
     enum Example {
         StructVariant { id: i32, name: String },
     }
     ```

3. **Associated Data**:
   - Variants like `Burrito` and `Bowl` use associated enums (`Meat` and `Beans`) to define their fields.

4. **Nesting Enums**:
   - Enums can reference other enums, as seen in `Burrito { meat: Meat::Steak, beans: Beans::Pinto }`.

5. **Debug Formatting**:
   - Deriving the `Debug` trait (`#[derive(Debug)]`) allows easy inspection of enums and their associated data.

### Output
When you run the program, it will produce:

```
Lunch was: Burrito { meat: Steak, beans: Pinto }.
Dinner was: Bowl { meat: Chicken, beans: Black }.
Nobody ate: VeganPlate.
```

### Use Cases:
- **Real-world modeling**: Perfect for modeling systems like restaurant menus, file formats, or state machines.
- **Flexible and extensible**: New variants or fields can be added without breaking existing code.
