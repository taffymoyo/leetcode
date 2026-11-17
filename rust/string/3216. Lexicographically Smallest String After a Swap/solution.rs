impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut arr: Vec<char> = s.chars().collect(); // string into list

        for i in 0..arr.len() - 1 {
            if arr[i].to_digit(10).unwrap() % 2 == 0 && arr[i + 1].to_digit(10).unwrap() % 2 == 0 { // both even
                if arr[i + 1] < arr[i] {
                    arr.swap(i, i + 1); // using the niche swap method, python wishes it had ts 🥹 
                    break; 
                }
            } else if arr[i].to_digit(10).unwrap() % 2 != 0 && arr[i + 1].to_digit(10).unwrap() % 2 != 0 {
                if arr[i + 1] < arr[i] {
                    arr.swap(i, i + 1);
                    break;
                }   
            }
        }
        arr.into_iter().collect()
    }   
}
