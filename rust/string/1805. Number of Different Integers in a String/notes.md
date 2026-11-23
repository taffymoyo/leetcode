# 1805. Number of Different Integers in a String
## What I learned
* Build each run of digits into a String, and when a non-digit ends the run, insert it into a HashSet. A set keeps only distinct values, so seen.len() is the answer
* Leading zeros have to be stripped before inserting, so "01" and "1" count as the same integer. trim_start_matches('0') does that
* A run of only zeros, like "0" or "00", trims to an empty string. That is still consistent, since every all-zero run becomes "" and counts once
* String::push takes a char, and += takes a &str (not just string literals). Pushing a single char from chars() is why push is used here
* The last run of digits has no non-digit after it to end it, so it needs a check after the loop
* trim_start_matches returns a &str that borrows from int, so .to_string() makes an owned copy before the set stores it and int is cleared
* `seen.len() as i32` casts the usize length to the i32 return type
## Approach
1. Walk through the characters, pushing digits onto a growing string
2. When a non-digit follows a digit run, strip leading zeros, insert the result into the set, and clear the string
3. After the loop, insert the final run if one is left
4. Return the size of the set
## Complexity
Time: O(n) where n is the length of word  
Aux Space: O(n) for the set and the current run
