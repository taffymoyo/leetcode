impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack: Vec<i32> = vec![];
        let mut nums = nums;
        nums.sort();

        for num in &nums {
            if stack.len() < 2 {
                stack.push(*num)
            }
            
            if stack.len() == 2 {
                result.push(stack.pop().unwrap());
                result.push(stack.pop().unwrap());
            }
        }
        result
    }
}
