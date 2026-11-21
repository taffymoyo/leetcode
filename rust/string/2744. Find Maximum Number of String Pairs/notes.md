# 2744. Find Maximum Number of String Pairs
## What I learned
* A HashSet of the words seen so far turns "is there an earlier word that is my reverse?" into an O(1) lookup, instead of a nested loop over every pair
* word.chars().rev().collect::<String>() reverses a string. chars() yields the characters, rev() makes the iterator walk from the back, and collect() builds them into a new String
* String literals like "cd" are &str, not String. The input here is a Vec<String>, so for word in &words gives &String
* HashSet<String>::contains takes a reference (&String), because it only looks at the value. Since rev is already a String, contains(&rev) works with no to_string()
* insert takes ownership of what it stores, so the word has to be cloned first (word.to_string() or word.clone()), because the loop only has a reference
## Approach
1. For each word, build its reverse
2. If the reverse is already in the set, an earlier word pairs with this one, so add 1 to the result
3. Otherwise insert the word into the set so later words can pair with it
4. Return the result
## Complexity
Time: O(n * m) where n is the number of words and m is the word length (m = 2 here)
Aux Space: O(n) for the set
