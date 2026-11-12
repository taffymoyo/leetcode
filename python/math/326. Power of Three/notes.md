# 326. Power of Three

## What I learned
- Repeatedly dividing a number by its base can determine whether it is a power of that base.
- Using a loop allows us to break a larger number down until it reaches a smaller value.
- The final value must be exactly 1 or 3 for the number to be a power of three.

## Approach
1. While `n` is greater than 3, repeatedly divide it by 3.
2. Once the loop finishes, check whether the remaining value is either 1 or 3.
3. Return `True` if either condition holds, otherwise return `False`.

## Complexity
- **Time:** O(log₃ n), as we divide the number by 3 on each iteration.
- **Aux Space:** O(1), as we only use a constant amount of additional space.
