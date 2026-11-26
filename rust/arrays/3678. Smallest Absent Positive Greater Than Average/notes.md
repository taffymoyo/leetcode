# 3678. Smallest Absent Positive Greater Than Average
## What I learned
* Rust integer division truncates toward zero, while Python's // floors toward negative infinity. They only differ for negative results
* Truncating the average is safe here. For an integer i, `i > average` gives the same answer whether average is the exact decimal or its truncated value, and any positive i beats a negative average anyway
* `for i in 1..` is an open-ended range that counts up forever, so the loop only ends through the return inside it
* The final `0` is never reached, but the compiler can't tell the loop is infinite, so the function still needs a value there
* nums.len() is a usize, so it needs `as i32` before dividing an i32 by it
* HashSet::contains takes a reference, so it's `&i`, and for num in &nums gives &i32, so inserting needs `*num`
## Approach
1. Sum the numbers while putting each one in a HashSet
2. Divide the sum by the length to get the average
3. Count up from 1 and return the first i that isn't in the set and is greater than the average
## Complexity
Time: O(n + m), where m is how far the search counts past the average  
Aux Space: O(n) for the set
