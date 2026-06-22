# Solution notes: Longest word slice

## Approach

1. **Initialize:** Use an empty string slice as the longest word tracker
2. **Iterate:** Loop through all whitespace-separated words using `.split_whitespace()`
3. **Compare lengths:** For each word, check if its length is **strictly greater** than the current longest
4. **Update only on strictly greater:** This ensures ties go to the first occurrence
5. **Check for empty:** After the loop, if longest is empty, return `None`; otherwise return `Some(&longest)`

### Example: `"the quick brown fox"`
- "the" (3) → longest = "the"
- "quick" (5) → 5 > 3, longest = "quick"
- "brown" (5) → 5 is NOT > 5, skip (tie keeps first)
- "fox" (3) → 3 is NOT > 5, skip
- Result: `Some("quick")` ✓

## Edge cases handled

- **Empty string:** Returns `None`
- **Whitespace only:** `split_whitespace()` produces no words → returns `None`
- **Single word:** Returns `Some(word)`
- **Ties:** First occurrence wins (due to `>` not `>=`)
- **Multiple spaces:** `split_whitespace()` automatically collapses multiple spaces

## Anything special

- **Borrowed slice return:** Must return `&str` (borrowed), not `String` - satisfies ownership constraint
- **split_whitespace():** Perfect for this - skips empty strings and handles all whitespace types
- **Comparison matters:** Using `>` instead of `>=` ensures deterministic behavior on ties

_Tricks, alternatives you considered, performance notes, etc._
