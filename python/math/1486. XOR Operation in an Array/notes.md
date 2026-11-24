# 1486. XOR Operation in an Array

## What I learned
- `reduce()` repeatedly applies a function to combine iterable elements into one result.
- The third argument of `reduce()` specifies the initial value.
- XOR (`^`) can be used to combine multiple integers into a single result.
- `0 ^ x = x`, making `0` a suitable initial value.

## Approach
1. Generate an array of `n` elements using the formula `start + 2 * i`.
2. Use `reduce()` with the XOR operator to combine all elements.
3. Return the resulting XOR value.

## Complexity
- **Time:** \(O(n)\) — generate and XOR all elements.
- **Auxiliary Space:** \(O(n)\) — store the generated array.
