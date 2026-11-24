# 657. Robot Return to Origin

## What I learned
- Lists are mutable, allowing individual elements to be updated.
- A 2D position can be represented using `[x, y]`.
- Comparing lists with `==` checks whether their elements match.

## Approach
1. Initialise the robot's position at `[0, 0]`.
2. Iterate through each movement in the string.
3. Update the x or y coordinate depending on the direction:
   - `U`: increase y.
   - `D`: decrease y.
   - `R`: increase x.
   - `L`: decrease x.
4. Check whether the final position is `[0, 0]`.
5. Return the result.

## Complexity
- **Time:** \(O(n)\) — iterate through each movement once.
- **Auxiliary Space:** \(O(1)\) — only maintain two coordinates.
