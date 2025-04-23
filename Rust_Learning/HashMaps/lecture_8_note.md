# HashSet Operations



### 🔄 Comparing HashSets

HashSets can be compared using **set operations**, just like in math.

#### 🧾 Example Sets:
```rust
let mut concert_queue: HashSet<&str> = HashSet::new();
concert_queue.insert("Boris");
concert_queue.insert("Melissa");

let mut movie_queue: HashSet<&str> = HashSet::new();
movie_queue.insert("Boris");
movie_queue.insert("Phil");
```

### ✅ Methods to Compare Sets:

| Method | Description | Example Output |
|--------|-------------|----------------|
| `union()` | All unique values from both sets | `["Boris", "Melissa", "Phil"]` |
| `difference()` | Values in set A, not in set B | `concert_queue.difference(&movie_queue)` → `["Melissa"]` |
| `symmetric_difference()` | Values in either set, but not both | `["Melissa", "Phil"]` |
| `is_disjoint()` | True if no common values | Returns `false` here (both have "Boris") |
| `is_subset()` | True if all values in set A are in set B | e.g. `attendees.is_subset(&concert_queue)` |
| `is_superset()` | True if all values in set B are in set A | e.g. `concert_queue.is_superset(&attendees)` |

> These methods return special structs (like `Union`, `Difference`, etc.) but you can print them using `{:?}`.

---

### 📌 Tip:
- **HashSets are unordered** – don’t expect a specific order when printing results.
- These operations are useful for **filtering, searching, and analyzing relationships** between sets.
