impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {

        // note, double quote strings such as "cd" are always String type, not &str
        // single quote strings such as 'a' are of the char type (must be exactly one character)
        // chars are always 4 bytes (32 bits)
        
        use std::collections::HashSet;
        let mut res: i32 = 0;
        let mut set = HashSet::new();

        for word in &words { // each word is an &str
            let rev: String = word.chars().rev().collect(); 
            // chars() creates a chars iterator (with l and r pointers)
            // rev() uses the right pointer, next() goes right to left
            // after each next() and char is yielded, collect() builds it onto the heap as a String

            if set.contains(&rev.to_string()) { // contains only takes references since it is a read only operation
                res += 1;
            } else {
                set.insert(word.to_string()); // to_string() converts specified value to String type
            }
        }

        res

        }
    }
