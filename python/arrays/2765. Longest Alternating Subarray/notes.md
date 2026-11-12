# 2765. Longest Alternating Subarray
## What I learned
* A valid subarray follows the pattern a, a+1, a, a+1, ..., so from any start index each next element is either nums[i] + 1 or nums[i] again
* The first match brings the length to 2, which is why count jumps from -1 to 2
* Return -1 if no start index gets past a length of 1
## Approach
1. For each start index i, walk j forward, expecting nums[i] + 1, then nums[i], then nums[i] + 1, and so on
2. Track the longest run found, and break as soon as the pattern fails
3. longest stays -1 if no alternating subarray exists
## Complexity
Time: O(n²)  
Aux Space: O(1)