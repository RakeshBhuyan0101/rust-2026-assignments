# Solution notes: Inventory

## Approach

### `restock()` function:
1. **Create HashMap:** `HashMap<String, u32>` to track items and quantities
2. **Chain both vectors:** Use `.into_iter().chain(more)` to iterate both inventories
3. **Accumulate counts:** For each item:
   - Use `.entry(name).or_insert(0)` to get or create entry
   - Increment quantity: `*map.entry(name).or_insert(0) += qty`
4. **Convert to Vec:** Use `.into_iter().collect()` to turn HashMap back to Vec
5. **Return:** The merged inventory (order unspecified)

### `summary()` function:
1. **Count items:** Use `inventory.len()` to get number of distinct items
2. **Sum quantities:** Use `inventory.iter().map(|(_, qty)| qty).sum()` to total units
3. **Format string:** Return `format!("{} items, {} units", count, total)`

### Example:
Input:
- `inventory`: [(“apple”, 3), (“banana”, 1)]
- `more`: [(“apple”, 2), (“orange”, 4)]

restock result: [(“apple”, 5), (“banana”, 1), (“orange”, 4)] ✓

## Edge cases handled

- **Both empty:** Returns empty vector
- **One empty:** Returns the non-empty list
- **Duplicates across both:** Quantities correctly sum
- **Empty summary:** Returns “0 items, 0 units”
- **Single item:** Works correctly

## Anything special

- **Ownership constraint satisfied:** `summary()` only borrows, `restock()` consumes both vectors
- **Chain efficiency:** `.chain()` without materializing intermediate list
- **HashMap for merging:** O(n) efficient deduplication and summing
- **Type preservation:** Original `String` types maintained throughout

_Tricks, alternatives you considered, performance notes, etc._
