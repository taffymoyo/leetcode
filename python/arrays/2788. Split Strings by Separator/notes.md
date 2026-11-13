# 2788. Split Strings by Separator
## What I learned
* str.split(separator) splits a string into a list of pieces, and consecutive or leading/trailing separators produce empty strings ("a..b".split(".") gives ["a", "", "b"])
* A nested comprehension flattens a list of lists: [x for sublist in lists for x in sublist]. The for clauses read in the same order as nested loops, outer first
* The trailing `if word` filters out empty strings, since an empty string is falsy
## Approach
1. Split every word by the separator, which gives a list of lists
2. Flatten into one list while skipping empty strings
3. Order is kept, since both steps go through the words left to right
## Complexity
Time: O(n) where n is the total number of characters across all words
Aux Space: O(n) for the split pieces and the flattened result
