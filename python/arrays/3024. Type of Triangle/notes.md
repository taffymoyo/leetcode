# 3024. Type of Triangle

## What I learned
- A `set()` removes duplicate values, making it useful for counting distinct elements.
- The triangle inequality determines whether three sides can form a valid triangle.
- All three triangle inequalities must hold for a valid triangle.

## Approach
1. Convert the side lengths into a set to identify the number of distinct values.
2. Check the triangle inequality for all three combinations of sides.
3. If any inequality fails, return `"none"`.
4. Otherwise, determine the triangle type:
   - 1 distinct value → `"equilateral"`
   - 2 distinct values → `"isosceles"`
   - 3 distinct values → `"scalene"`

## Complexity
- **Time:** \(O(1)\) — only three side lengths are processed.
- **Auxiliary Space:** \(O(1)\) — the set contains at most three elements.
