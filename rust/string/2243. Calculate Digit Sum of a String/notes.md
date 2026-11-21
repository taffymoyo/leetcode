# 2243. Calculate Digit Sum of a String
## What I learned
* Each round splits s into chunks of k (the last may be shorter), replaces each chunk with its digit sum, and joins them. Repeat until s.len() <= k
* s.len() is a usize, so k needs `k as usize`
* &s[i..j] gives a &str that borrows from s
* to_digit(10) returns an Option<u32>, so it needs unwrap()
* `res += &seg_total.to_string()` appends to a String, since += takes a &str
## Approach
1. While s is longer than k, start a round with an empty result string
2. Walk s in steps of k, taking each segment
3. Sum the digits in the segment and append the total to the result
4. Copy the result into s and return it once len <= k
## Complexity
Time: O(n * r) for n = len(s) and r rounds (n <= 100, so tiny)
Aux Space: O(n)
