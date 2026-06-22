# Solution notes: Split and double

## Approach

1. **Double all elements:** Use `for e in xs.iter_mut()` to mutably iterate
   - Each element: `*e = *e * 2`
   - Uses `iter_mut()` to borrow (not move) `xs`
2. **Split at mid:** Use `xs.split_at_mut(mid)` to create two disjoint mutable slices
   - Returns `(&mut [i32], &mut [i32])`
3. **Return both slices:** Return the split result as a tuple

### Example: `[1, 2, 3, 4]` with `mid=2`
- Double: [1, 2, 3, 4] → [2, 4, 6, 8]
- split_at_mut(2): left=[2, 4], right=[6, 8]
- Return: `(&mut [2, 4], &mut [6, 8])` ✓

## Edge cases handled

- **Empty vector:** Returns `(&mut [], &mut [])`
- **mid = 0:** Returns `(&mut [], &mut [...all elements...])`
- **mid = len:** Returns `(&mut [...all elements...], &mut [])`
- **mid > len:** Panics (standard library behavior)

## Anything special

- **Key insight:** Can't use `for e in xs` (would move); must use `for e in xs.iter_mut()` (borrows)
- **split_at_mut benefit:** Creates truly disjoint mutable references - compiler allows this
- **In-place doubling:** No allocation needed, modifies vector directly
- **Ownership preserved:** `xs` remains available after loop because `iter_mut()` only borrows

_Tricks, alternatives you considered, performance notes, etc._
