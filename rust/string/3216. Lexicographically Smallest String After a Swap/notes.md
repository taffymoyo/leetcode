# 3216. Lexicographically Smallest String After a Swap
## What I learned
* s.chars().collect() turns a String into a Vec<char>, and arr.into_iter().collect() turns it back into a String. The type annotation or the return type tells collect() what to build
* char::to_digit(10) returns an Option<u32> (None if the char isn't a digit), so it needs unwrap() to get the number
* Vec::swap(i, j) swaps two elements in place, with no temporary variable
* Only one swap is allowed, so the first adjacent same-parity pair where the right digit is smaller gives the best result. Swapping earlier makes a smaller digit land in a more significant position, so break straight after
## Approach
1. Convert the string to a Vec<char> so it can be edited in place
2. Scan adjacent pairs from left to right
3. If both digits have the same parity and the right one is smaller, swap them and stop
4. Convert back to a String and return
## Complexity
Time: O(n)  
Aux Space: O(n) for the char vector
