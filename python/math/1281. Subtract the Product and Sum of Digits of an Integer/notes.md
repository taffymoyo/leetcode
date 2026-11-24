# 1281. Subtract the Product and Sum of Digits of an Integer

## What I learned
- `str(n)` converts an integer into a string, allowing iteration over individual digits.
- `int(digit)` converts each character back into an integer.
- Multiple accumulator variables can track different calculations in one loop.

## Approach
1. Initialise `product` to `1` and `summ` to `0`.
2. Convert the number into a string and iterate through each digit.
3. Multiply each digit into `product`.
4. Add each digit to `summ`.
5. Return the product minus the sum.

## Complexity
- **Time:** \(O(\log n)\) — iterate through each digit.
- **Auxiliary Space:** \(O(\log n)\) — converting the integer to a string.
