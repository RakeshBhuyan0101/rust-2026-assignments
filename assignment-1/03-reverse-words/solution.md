# Solution notes: Reverse the word order

## Approach

1. **Split:** Use `.split_whitespace()` to get individual words (collapses multiple spaces)
2. **Reverse:** Use `.rev()` to reverse the iterator order
3. **Collect:** Use `.collect()` to turn words into a `Vec<&str>`
4. **Join:** Use `.join(" ")` to combine words with single spaces
5. **Return:** The resulting `String`

One-liner approach:
```rust
sentence
    .split_whitespace()
    .rev()
    .collect::<Vec<_>>()
    .join(" ")
```

### Example: `"   one   two  "`
- split_whitespace: ["one", "two"]
- rev: ["two", "one"]
- collect: Vec<&str>
- join with " ": "two one" ✓

## Edge cases handled

- **Empty string:** Returns empty string
- **Whitespace only:** No words to reverse → returns empty string
- **Single word:** Reversed to itself
- **Multiple spaces collapsed:** Replaced with single space in output
- **Leading/trailing spaces removed:** `split_whitespace()` automatically trims

## Anything special

- **Chain operations:** Efficient pipeline without intermediate allocations
- **join() for spacing:** Ensures exactly one space between words
- **split_whitespace() efficiency:** Single pass, handles all whitespace types (spaces, tabs, newlines)

_Tricks, alternatives you considered, performance notes, etc._
