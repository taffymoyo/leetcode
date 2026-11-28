# 1021. Remove Outermost Parentheses (Rust)

## What I learned
- `Vec<char>` can be used as a stack in Rust.
- `.push()` adds an element to the end of a vector.
- `.pop()` removes the last element.
- `.len()` checks the current number of elements.
- Single quotes (`'('`) represent `char`, while double quotes (`"("`) represent string slices.
- `.iter().collect()` can collect characters into a `String`.

## Approach
1. Initialise two vectors: `result` to store the remaining parentheses and `stack` to track nesting depth.
2. Iterate through each character using `.chars()`.
3. When encountering `'('`, add it to the result only if the stack is already non-empty, then push it onto the stack.
4. When encountering `')'`, pop from the stack first, then add it to the result only if the stack remains non-empty.
5. Collect the result vector into a `String` and return it.

## Complexity
- **Time:** \(O(n)\) — iterate through each character once.
- **Auxiliary Space:** \(O(n)\) — the stack and result can both grow proportionally to the input length.
