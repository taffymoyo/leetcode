impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        
        match timer { // cleaner way to write if / elif / else chains
            0 => "Green",
            30 => "Orange",
            31..=90 => "Red",
            _ => "Invalid",
        }.to_string()
        // no semicolon means the result of the match is returned
        // almost every curly brace block {} evaluates to an expression, the no semi colon can be used at the end of any curly brace block
        // code with semi colons after it are statements not expressions
    }
}
