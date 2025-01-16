### Returning an option enum from a function

```rust
fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true) // Item exists in the catalog and is in stock
    } else if item_is_in_system {
        Some(false) // Item exists in the catalog but is not in stock
    } else {
        None // Item does not exist in the catalog
    }
}

fn main() {
    let availability = is_item_in_stock(true, true);
    match availability {
        Some(true) => println!("Yes, the item is available."),
        Some(false) => println!("No, the item is not in stock."),
        None => println!("Your item doesn't exist in our system."),
    }

    let availability = is_item_in_stock(true, false);
    match availability {
        Some(true) => println!("Yes, the item is available."),
        Some(false) => println!("No, the item is not in stock."),
        None => println!("Your item doesn't exist in our system."),
    }

    let availability = is_item_in_stock(false, false);
    match availability {
        Some(true) => println!("Yes, the item is available."),
        Some(false) => println!("No, the item is not in stock."),
        None => println!("Your item doesn't exist in our system."),
    }
}
```

### Explanation of Key Components

1. **Function Definition**:
   - The `is_item_in_stock` function takes two `bool` parameters: `item_is_in_system` and `item_is_in_stock`.
   - It returns an `Option<bool>`. 
     - `Some(true)` indicates the item is available.
     - `Some(false)` indicates the item exists but is not in stock.
     - `None` indicates the item does not exist in the catalog.

2. **Logic**:
   - If the item is in the system and in stock: return `Some(true)`.
   - If the item is in the system but not in stock: return `Some(false)`.
   - If the item is not in the system: return `None`.

3. **`main` Function**:
   - The `availability` variable stores the result of calling `is_item_in_stock`.
   - A `match` statement is used to handle all possible return values of `Option<bool>`, covering:
     - `Some(true)`
     - `Some(false)`
     - `None`

4. **Dynamic Handling with Match Arms**:
   - For `Some(true)`, the output is "Yes, the item is available."
   - For `Some(false)`, the output is "No, the item is not in stock."
   - For `None`, the output is "Your item doesn't exist in our system."

### Example Outputs
#### Input: `(true, true)`
```
Yes, the item is available.
```

#### Input: `(true, false)`
```
No, the item is not in stock.
```

#### Input: `(false, false)`
```
Your item doesn't exist in our system.
```

