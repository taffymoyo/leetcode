# 2894. Divisible and Non-divisible Sums Difference

## What I learned
- The modulo operator (`%`) checks divisibility.
- Two accumulator variables can track separate sums.
- An `if/else` statement separates numbers into two groups.

## Approach
1. Initialise two variables, `num1` and `num2`, to store the sums.
2. Iterate through every number from `1` to `n`.
3. If the number is divisible by `m`, add it to `num2`.
4. Otherwise, add it to `num1`.
5. Return `num1 - num2`.

## Complexity
- **Time:** \(O(n)\) — iterate through every number once.
- **Auxiliary Space:** \(O(1)\) — only use two accumulator variables.
