# 1572. Matrix Diagonal Sum
## What I learned
* For odd-sized matrices the center element sits on both diagonals and must be subtracted once to avoid double counting
* Both diagonals can be accumulated in a single loop: row[i] for primary, row[n-1-i] for secondary
## Approach
1. If n is odd, preemptively subtract the center element to cancel the double count
2. Loop through rows, adding both diagonal elements per row
3. Return count
## Complexity
Time: O(n) where n is the number of rows  
Aux Space: O(1)
## Solution
class Solution:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        
        count = 0
        n = len(mat)
        if n % 2 == 1:
            count -= mat[n//2][n//2]

        for i, row in enumerate(mat):
            count += row[i]
            count += row[n-1-i]
        
        return count
