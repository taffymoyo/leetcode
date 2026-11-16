impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        
        let mut j = 1;
        for i in 0..nums.len() - 1 {
            if nums[i] % 2 == 0 {
                if nums[j] % 2 == 0 {
                    return false; 
                    // every block {} must evaluate to an something, even (). 
                    // false by itself doesnt work inside an if statement, both types in the branches of the if and else must produce the same type.
                    // an if with no else must produce (), because the "invisible" else evaluates to () aswell.
                }
            } else {
                if nums[j] % 2 == 1 {
                    return false;
                }
            }
            j += 1
        }
        true
    }
}
