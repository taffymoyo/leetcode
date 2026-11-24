# 2652. Sum Multiples

## What I learned
- The modulo operator (`%`) checks whether a number is divisible by another number.
- Using `or` allows checking multiple conditions.
- A number satisfying multiple conditions is still only counted once.

## Approach
1. Iterate through every number from `1` to `n`.
2. Check whether the number is divisible by `3`, `5`, or `7`.
3. If any condition is true, add the number to `count`.
4. Return the total sum.

## Complexity
- **Time:** \(O(n)\) — iterate through the range once.  
- **Auxiliary Space:** \(O(1)\) — only use a single counter.
