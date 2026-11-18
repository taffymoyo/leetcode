# 2923. Find Champion I
## What I learned
* Team i is the champion if no other team beats it, which means every grid[i][j] with j != i is 1. A single 0 in row i means team j is stronger, so row i can't be the champion
* iter().enumerate() gives (index, &item), so `* num == 0` dereferences the reference before comparing, the same as the *num in the earlier problems
* A flag plus break stops scanning a row as soon as one 0 is found, then the flag decides whether to return that row
* `i as i32` casts the usize index to the i32 return type, since Rust won't convert between integer types automatically
* The final -1 is the function's return value, with no semicolon. It's never reached if the input is valid
## Approach
1. For each team i, scan its row
2. If any grid[i][j] with j != i is 0, team i loses to someone, so stop scanning
3. If the whole row had no 0s, team i is the champion, so return i
4. Return -1 if nothing is found
## Complexity
Time: O(n²) worst case, since each row is scanned until its first 0
Aux Space: O(1)
