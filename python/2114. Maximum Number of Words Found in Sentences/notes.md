# 2114. Maximum Number of Words Found in Sentences

## What I learned
* `split()` separates a string into a list of words using spaces
* `len()` returns the number of words in the resulting list
* `max()` can compare the current count against the maximum found so far

## Approach
1. Initialise `maximum` to 0
2. Iterate through each sentence
3. Split each sentence into words and calculate its length
4. Update maximum if a larger count is found
5. Return maximum

## Complexity
Time: O(n × m) — n sentences with an average length of m  
Aux Space: O(m) — `split()` creates a list of words for each sentence
