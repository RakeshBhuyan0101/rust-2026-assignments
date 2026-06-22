# Solution notes: Caesar cipher

## Approach

1. **Iterate through each character** in the input string
2. **Check character type:**
   - If lowercase letter: shift within 'a'-'z' range
   - If uppercase letter: shift within 'A'-'Z' range
   - If non-letter: keep unchanged
3. **For letter shifts:**
   - Find position in alphabet: `(char - base) as i32`
   - Apply shift: `(position + shift).rem_euclid(ALPHABET.len() as i32)`
   - Use `rem_euclid()` to handle negative shifts and wraps > 26 correctly
   - Look up new character in ALPHABET
4. **Build result string** by pushing each processed character

### Example: `caesar("abc", -1)`
- 'a' (pos 0): (0 + (-1)) % 26 = 25 → 'z'
- 'b' (pos 1): (1 + (-1)) % 26 = 0 → 'a'
- 'c' (pos 2): (2 + (-1)) % 26 = 1 → 'b'
- Result: "zab" ✓

## Edge cases handled

- **Negative shifts:** `rem_euclid()` wraps correctly (e.g., -1 maps to 25)
- **Large shifts:** 27 is equivalent to 1 (wraps around using modulo)
- **Mixed case:** Uppercase and lowercase letters handled separately
- **Non-letters:** Digits, punctuation, whitespace preserved unchanged
- **Empty input:** Returns empty string

## Anything special

- **Key optimization:** Used `rem_euclid()` instead of `%` operator because Rust's `%` can return negative values for negative operands, while `rem_euclid()` always returns positive remainders (0..26)
- **Used ALPHABET const:** Followed constraint to reference ALPHABET rather than hard-coding 26
- **Character lookup:** Used `ALPHABET.chars().nth(index)` to map position back to character
