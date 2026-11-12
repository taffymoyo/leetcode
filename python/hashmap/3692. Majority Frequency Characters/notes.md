# 3692. Majority Frequency Characters

## What I learned
* `Counter()` counts how often each character appears in a string
* A dictionary can group characters by their frequency
* Track how many distinct characters belong to each frequency group
* When group sizes tie, choose the group with the larger frequency
* Iterate through the original counter to collect characters belonging to the winning group

## Approach
1. Use `Counter()` to count each character's frequency
2. Build a dictionary mapping each frequency to the number of distinct characters with that frequency
3. Find the frequency group with the largest size, breaking ties using the larger frequency
4. Iterate through the counter and collect characters belonging to the winning frequency
5. Return the resulting string

## Complexity
Time: O(n) — counting, grouping and iterating through characters  
Aux Space: O(1) — at most 26 lowercase English characters
