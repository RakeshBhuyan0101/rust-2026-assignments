# Solution notes: Character frequency, sorted

## Approach

1. **Build frequency map:** Use `HashMap<char, u32>` to count occurrences
2. **Iterate and count:** For each character, use `.entry().or_insert(0)` to increment count
3. **Convert to Vec:** Use `.into_iter().collect()` to turn HashMap into Vec of tuples
4. **Sort:** Use `.sort_by()` with custom comparator:
   - **Primary:** Sort by count descending (`b.1.cmp(&a.1)`)
   - **Secondary:** Sort by character ascending (`a.0.cmp(&b.0)`)
5. **Return:** Sorted vector

### Example: `"mississippi"`
- Map: {'m': 1, 'i': 4, 's': 4, 'p': 2}
- Convert to Vec: [('m', 1), ('i', 4), ('s', 4), ('p', 2)]
- Sort by count desc, then char asc:
  - ('i', 4) and ('s', 4) tied → sort alphabetically → 'i' < 's'
  - ('p', 2)
  - ('m', 1)
- Result: [('i', 4), ('s', 4), ('p', 2), ('m', 1)] ✓

## Edge cases handled

- **Empty string:** Returns empty vector
- **Single character:** Returns `[(char, 1)]`
- **All same character:** Returns `[(char, length)]`
- **Tied frequencies:** Secondary sort by character alphabetically ensures deterministic order
- **All different frequencies:** Simply sorted by count descending

## Anything special

- **HashMap efficiency:** O(n) build time with `.entry().or_insert()`
- **Chained comparisons:** `.then()` allows multiple sort criteria
- **Descending count:** Use `b.1.cmp(&a.1)` (flip arguments) for descending order
- **Ascending character:** Use `a.0.cmp(&b.0)` for ascending alphabetical order

_Tricks, alternatives you considered, performance notes, etc._
