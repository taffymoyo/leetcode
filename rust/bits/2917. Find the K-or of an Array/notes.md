## What I learned
* `vec![value; size]` initialises a vector with a repeated value
* `format!("{:031b}", num)` converts an integer to a 31-bit binary string, padding with leading zeros
* `.chars().enumerate()` iterates through characters with their indices without needing an additional reference
* `iter()` borrows vector elements, so `*bit` dereferences the value
* `1 << i` shifts a binary 1 into the required bit position
* `|=` performs bitwise OR and updates the result

## Approach
1. Create a vector of 31 zeros to count occurrences of each bit position
2. Convert each number into a 31-bit binary string
3. Iterate through each binary digit and increment its corresponding bit count if it is `1`
4. Iterate through the bit counts and check whether each position appears at least `k` times
5. Use bitwise OR to set qualifying positions in the result
6. Return the result

## Complexity
Time: O(n × 31) — process 31 bits for each number  
Aux Space: O(1) — fixed-size bit count vector
