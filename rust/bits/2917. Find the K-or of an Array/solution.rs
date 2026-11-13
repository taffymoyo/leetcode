impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut bit_count: Vec<i32> = vec![0; 31];

        for num in &nums {
            let bin_num = format!("{:031b}", num);
            for (i, digit) in bin_num.chars().enumerate() {
                if digit == '1' {
                    bit_count[30 - i] += 1;
                }
            }
        }
        
        let mut result = 0;
        for (i, bit) in bit_count.iter().enumerate() {
            if *bit >= k {
                result |= 1 << i;
            }
        }

        result
    }
}
