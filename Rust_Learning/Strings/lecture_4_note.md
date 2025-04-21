# Common String Methods (trim, casing, replace, split)


#### 🔤 `trim()`, `trim_start()`, `trim_end()`
- Remove whitespace:
  - `trim()` – both ends
  - `trim_start()` – beginning only
  - `trim_end()` – end only
- Returns a `&str` (no heap allocation).

```rust
let cleaned = music_genres.trim();
```

---

#### 🔠 `to_uppercase()` / `to_lowercase()`
- Convert all letters to upper/lower case.
- Returns a new `String` (heap allocated).

```rust
let upper = music_genres.to_uppercase();
let lower = music_genres.to_lowercase();
```

---

#### 🔁 `replace(from, to)`
- Replace all occurrences of a substring.

```rust
let funny = music_genres.replace("a", "@");
```

---

#### 🔪 `split(delimiter)`
- Split string into pieces by delimiter.
- Returns an iterator (`Split` struct).
- Use `.collect::<Vec<&str>>()` to get a `Vec`.

```rust
let genres: Vec<&str> = music_genres.split(", ").collect();
println!("{:?}", genres); // ["Rock", "Metal", "Country", "Rap"]
```

---

📌 **Tip**: These methods help clean, normalize, and process incoming string data easily (e.g., from files or user input).
