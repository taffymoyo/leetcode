# 3151. Special Array I
## What I learned
* In Rust, an `if` without an `else` must evaluate to `()`, so a bare `false` inside one is a type error. Use `return false;` to exit the function early
* The final `true` with no semicolon is the function's return value, reached only if no pair matched
* Two numbers have the same parity when `a % 2 == b % 2`, so the two branches collapse into one condition
## Approach
1. Walk through each adjacent pair (i, j = i + 1)
2. If both are even or both are odd, return false
3. If no pair has matching parity, return true
## Complexity
Time: O(n)
Aux Space: O(1)
## Solution
