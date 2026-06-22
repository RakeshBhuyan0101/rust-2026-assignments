# Solution notes: `min_max`

## Approach

1. **Handle empty slice:** Check with `if xs.is_empty() { return None; }`
2. **Initialize:** Set both `min` and `max` to the first element
3. **Single loop:** Iterate through all elements with `for &i in xs`
4. **Track minimum:** Update `min` if current element is smaller
5. **Track maximum:** Update `max` if current element is larger
6. **Return:** `Some((min, max))`

### Example: `&[3, 1, 4, 1, 5, 9, 2, 6]`
- Initialize: min=3, max=3
- See 1: 1 < 3 → min=1
- See 4: 4 > 3 → max=4
- See 1: 1 is not < 1, skip
- See 5: 5 > 4 → max=5
- See 9: 9 > 5 → max=9
- See 2: 2 < 1? No
- See 6: 6 is not > 9, skip
- Result: `Some((1, 9))` ✓

## Edge cases handled

- **Empty slice:** Returns `None` (checked first)
- **Single element:** Returns `Some((elem, elem))`
- **All equal:** Returns `Some((value, value))`
- **Ascending order:** Correctly finds min=1, max=5
- **Descending order:** Correctly finds min and max
- **Negative numbers:** Works with negative i32 values

## Anything special

- **Single manual loop:** No iterator adapters - straightforward imperative approach
- **Two independent comparisons:** Uses `if` statements (not if-else) for clarity
- **Borrow in loop:** `for &i in xs` borrows each element, doesn't move it
- **Early return:** Avoids panic by checking empty before accessing xs[0]

_Tricks, alternatives you considered, performance notes, etc._
