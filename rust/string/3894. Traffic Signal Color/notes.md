# 3894. Traffic Signal Color

## What I learned
- `match` provides a cleaner alternative to long `if / else if / else` chains.
- Range patterns like `31..=90` include both endpoints.
- `_` acts as a catch-all pattern, similar to `else`.
- Rust blocks can evaluate to expressions.
- Omitting the semicolon from the final expression returns its value.
- `.to_string()` converts a string slice (`&str`) into an owned `String`.

## Approach
1. Use `match` to compare the timer against different values.
2. Return `"Green"` for `0`.
3. Return `"Orange"` for `30`.
4. Return `"Red"` for values from `31` to `90`, inclusive.
5. Use `_` to handle all other values as `"Invalid"`.
6. Convert the resulting string slice into a `String`.

## Complexity
- **Time:** \(O(1)\) — constant-time pattern matching.  
- **Auxiliary Space:** \(O(1)\) — constant additional space.
