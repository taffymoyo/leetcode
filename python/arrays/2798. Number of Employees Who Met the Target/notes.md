# 2798. Number of Employees Who Met the Target

## What I learned
* Iterate directly over a list when the index isn't needed
* Use `>=` to check whether a value meets or exceeds a target
* A counter can track how many elements satisfy a condition

## Approach
1. Initialise `count` to 0
2. Iterate through each employee's hours
3. If hours are greater than or equal to the target, increment count
4. Return count

## Complexity
Time: O(n) — iterate through each employee once  
Aux Space: O(1) — constant extra space
