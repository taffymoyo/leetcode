impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        
        use std::collections::HashSet;

        let mut seen: HashSet<i32> = HashSet::new(); // set for faster lookup
        let mut double: Vec<i32> = vec![];

        for num in &nums {
            if seen.contains(num) && !double.contains(num) { // && operator same as and in python, can also be used as a double reference (not used here)
                    double.push(*num); // push actual num not address
                } else {
                    seen.insert(*num);
                }
            }
        
        if !double.is_empty() {
            let mut result: i32 = double[0];
            for num in double.iter().skip(1) {
                result ^= *num // implemented naturally but just good practice
            }
            result
        } else {
            0
        }
    }
}
