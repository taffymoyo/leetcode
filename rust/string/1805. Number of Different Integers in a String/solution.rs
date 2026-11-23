impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        use std::collections::HashSet;

        let mut seen: HashSet<String> = HashSet::new();
        let mut digit_last: bool = false;
        let mut int: String = String::new(); // owned string since it will grow

        for c in word.chars() {
            if c.is_digit(10) { // growing new integer
                int.push(c); // because += only takes string literals (&str), use push instead
                digit_last = true;
            } else if !c.is_digit(10) && digit_last {
                seen.insert(int.trim_start_matches('0').to_string()); // removes leading 0s (what is this function name bruh)
                int.clear(); // wipes string to prepare for new integer
                digit_last = false;
            }
        }

        if int.len() != 0 { // handles last integer
            seen.insert(int.trim_start_matches('0').to_string()); 
        }

        seen.len() as i32
    }
}
