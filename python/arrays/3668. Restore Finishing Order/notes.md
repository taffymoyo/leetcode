# 3668. Restore Finishing Order

## What I learned
- The `in` operator checks whether an element exists inside a collection.
- Iterating through `order` preserves the original finishing order.
- `result.append(num)` adds matching elements to the result in the same order they appear in `order`.
- Checking membership in a list takes O(m) time on average per lookup, whereas a HashSet (Python's `set`) provides O(1) average lookup.
- Converting `friends` into a set can therefore improve performance when there are many elements to check.

## Approach
1. Initialise an empty result list.
2. Iterate through `order` from first to last.
3. Check whether each number exists in `friends`.
4. If it does, append it to `result`.
5. Return the result, which preserves the original finishing order.

## Complexity
- **Time:** O(n × m), where n is the length of `order` and m is the length of `friends`, because each list membership check can take O(m).
- **Aux Space:** O(n) for the output list.