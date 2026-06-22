# Solution notes: Group anagrams

## Approach

1. **Create a HashMap:** Maps sorted character signatures to lists of words
2. **For each word in input:**
   - Convert to lowercase
   - Sort its characters to create a unique signature
   - Add the original word (preserving case) to the group for that signature
3. **Extract groups:** Convert HashMap into a Vec of Vec<String>

### Example: `["Eat", "tea", "ate"]`
- "Eat" → lowercase: "eat" → sort: ['a','e','t'] → signature: "aet"
- "tea" → lowercase: "tea" → sort: ['a','e','t'] → signature: "aet"
- "ate" → lowercase: "ate" → sort: ['a','e','t'] → signature: "aet"
- All three map to "aet" → grouped together with original casing preserved: ["Eat", "tea", "ate"]

## Edge cases handled

- **Empty input:** Returns empty Vec
- **Single word:** Returns Vec with one group containing that word
- **Case insensitive:** Uppercase and lowercase letters treated as equivalent during grouping
- **Preserves original casing:** Output words maintain their original case from input
- **Input order preserved:** Words within each group appear in their original input order

## Anything special

- **Used HashMap for O(n) performance:** Single pass through words to build groups
- **Signature generation:** Convert word to lowercase, sort chars, and convert back to String for HashMap key
- **Input order within groups:** `entry(sig).or_insert_with(Vec::new).push(word)` ensures words in each group maintain input order
- **Group order unspecified:** HashMap iteration order doesn't matter for this problem
