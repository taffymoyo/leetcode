impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {

        let mut s = s;
        let k = k as usize;

        while s.len() > k { // rounds can continue, len() returns a usize
        let mut i: usize = 0;
        let mut res = String::new();

            while i < s.len() {

                let mut segment: &str = ""; // double quotes always &str type
                if s.len() - i < k {
                    segment = &s[i..s.len()];
                } else {
                    segment = &s[i..i + k];
                }

                let mut seg_total = 0;

                for num in segment.chars() { // iterate over segment, chars() yields char types
                    let new_digit = num.to_digit(10).unwrap(); // to digit gives option type, must unwrap
                    seg_total += new_digit;
                }

                res += &seg_total.to_string();
                i += k;
            }
            s.clone_from(&res); // clones the string while breaking it, in place method (clone for String types)
        } 
        s
    }
}
