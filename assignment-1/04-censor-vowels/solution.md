# Solution notes: Censor vowels in place

## Approach

1. **Convert to mutable bytes:** Use `.as_bytes_mut()` to get mutable access to string bytes
2. **Iterate:** Loop through each byte
3. **Check for vowels:** Compare each byte with vowel characters (case-insensitive)
4. **Replace:** If vowel, replace with '*'
5. **Return:** Modified string (in-place modification via mutable reference)

### Example: `"Hello World"`
- 'H' → not vowel → 'H'
- 'e' → vowel → '*'
- 'l' → not vowel → 'l'
- 'l' → not vowel → 'l'
- 'o' → vowel → '*'
- ' ' → not vowel → ' '
- 'W' → not vowel → 'W'
- 'o' → vowel → '*'
- 'r' → not vowel → 'r'
- 'l' → not vowel → 'l'
- 'd' → not vowel → 'd'
- Result: "H*ll* W*rld" ✓

## Edge cases handled

- **Uppercase vowels:** A, E, I, O, U → replaced with '*'
- **Lowercase vowels:** a, e, i, o, u → replaced with '*'
- **Non-letters:** Digits, punctuation, spaces → preserved unchanged
- **Empty string:** Returns empty string

## Anything special

- **In-place modification:** Uses mutable reference and `as_bytes_mut()` for efficiency
- **ASCII comparison:** Direct byte comparison works since vowels are ASCII characters
- **No allocations:** Modifies the original string without creating new String
- **Simple pattern matching:** Case-insensitive check with two comparisons (uppercase and lowercase)

_Tricks, alternatives you considered, performance notes, etc._
