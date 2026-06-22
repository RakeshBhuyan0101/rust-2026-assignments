# Solution notes: Run-length encode

## Approach

1. **Initialize:** Create an empty result vector to store `(char, count)` tuples
2. **Convert to iterator:** Use `chars().peekable()` to iterate through the string and look ahead
3. **Outer loop:** Process each character via `while let Some(current_char) = chars.next()`
4. **Track count:** Start with count = 1 for the current character
5. **Inner loop:** While the next character matches the current one:
   - Increment count
   - Consume the matching character with `chars.next()`
6. **Store group:** Push `(current_char, count)` to result
7. **Return:** The result vector

### Example: `"aaabbc"` → `[('a', 3), ('b', 2), ('c', 1)]`
- Read 'a', count=1 → peek 'a' (match), count=2, consume → peek 'a' (match), count=3, consume → peek 'b' (no match)
- Push ('a', 3), move to 'b'
- Read 'b', count=1 → peek 'b' (match), count=2, consume → peek 'c' (no match)
- Push ('b', 2), move to 'c'
- Read 'c', count=1 → no next char
- Push ('c', 1)

## Edge cases handled

- **Empty string:** Returns empty vector
- **Single character:** Returns `[(char, 1)]`
- **All identical:** Returns `[(char, length)]`
- **All different:** Returns list of single occurrences
- **Alternating runs:** Correctly handles "aabbaa" → [('a', 2), ('b', 2), ('a', 2)]

## Anything special

- **Key technique:** `peekable()` iterator allows looking at next character without consuming it
- **Why peek matters:** We check if next char matches before consuming - this is crucial for counting consecutive runs
- **No allocations in loop:** We only allocate the result vector once and push to it
- **No manual indexing:** Iterator-based approach is safe and idiomatic Rust
