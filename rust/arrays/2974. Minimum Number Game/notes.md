# 2974. Minimum Number Game
## What I learned
* Alice removes the smallest number and Bob removes the next smallest, but Bob appends his first. After sorting, each pair (a, b) with a <= b goes into the result as b, a, so the pairs come out swapped
* A stack reverses order, so pushing the pair and popping both gives b first, then a
* `for num in &nums` borrows, so num is a &i32 and push needs `*num`
* `stack.pop()` returns an Option, so it needs unwrap()
* `let mut nums = nums;` re-declares the parameter as mutable so it can be sorted in place
## Approach
1. Sort nums so the smallest numbers come first
2. Push numbers onto a stack two at a time
3. When the stack holds a pair, pop both. The larger comes off first (Bob), then the smaller (Alice), and both go into the result in that order
4. Return the result
## Complexity
Time: O(n log n), dominated by the sort  
Aux Space: O(n) for the result, and the stack never holds more than 2
