# 3158. Find the XOR of Numbers Which Appear Twice
## What I learned
* A HashSet gives fast "have I seen this before?" checks, and a second collection records the numbers that repeat
* Borrowing a collection (for num in &nums) gives &i32, so contains(num) takes it as-is and push needs *num
* XOR folds a list into one value: start from the first element, then ^= each remaining one. XOR with 0 changes nothing, so an empty list gives 0
* The final if/else has no semicolons, so each branch is the value of the function
## Approach
1. Walk through nums, tracking numbers seen so far in a HashSet
2. If a number is already seen and not yet recorded, push it onto double
3. XOR everything in double together
4. If nothing repeated, return 0
## Complexity
Time: O(n²) worst case, because double.contains() is a linear scan. The constraints cap n at 50 and each number appears at most twice, so it doesn't matter here
Aux Space: O(n) for the set and the list
