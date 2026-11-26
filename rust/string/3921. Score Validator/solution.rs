impl Solution {
    pub fn score_validator(events: Vec<String>) -> Vec<i32> {

        let mut score: i32 = 0;
        let mut counter: i32 = 0;

        for element in &events {
            match element.as_str() { // element is String type, needs to be &str, match also dereferences element automatically due to rust ergonomics.
                "W" => counter += 1,
                "WD" | "NB" => score += 1, // pattern or (one pipe |) to match multiple shapes the value (element) can take to one result
                other => {
                    if let Ok(num) = other.parse::<i32>() { 
                    score += num;
                    }
                    // turbofish operator, tells compiler what type to use without creating syntax ambiguities, in this case parse aims to parse into i32 type.
                }
            } // control flow blocks {} dont end with ;

            if counter >= 10 {
                return vec![score, counter];
            }
        }

        vec![score, counter]
    }
}
