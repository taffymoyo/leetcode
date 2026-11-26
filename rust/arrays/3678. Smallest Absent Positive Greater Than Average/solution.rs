impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {

        use std::collections::HashSet;
        
        let mut average: i32 = 0;
        let mut seen: HashSet<i32> = HashSet::new();

        for num in &nums {
            average += *num;
            seen.insert(*num);
        }

        average = average / nums.len() as i32; //  trunacates towards 0 (not floor division whcih goes towards infinity)

        for i in 1.. {
            if !seen.contains(&i) && i > average {
                return i;
            }
        }

        0

    }
}
