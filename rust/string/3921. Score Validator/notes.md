# 3921. Score Validator
## What I learned
* match on a String needs a &str, because the arms are string literals. element.as_str() makes that conversion. match doesn't dereference for you here
* `"WD" | "NB"` is an or-pattern: one pipe lets several patterns share one arm
* The last arm `other` is a catch-all that binds the value to a name, so it can be used inside the arm. Unlike `_`, it gives you the value
* parse::<i32>() is the turbofish form. It tells the compiler which type to parse into, with no ambiguity. It returns a Result, so `if let Ok(num)` handles the success case and skips anything that fails to parse
* Match arms can be expressions like `counter += 1` or blocks {}
* The early return inside the loop exits as soon as the counter hits 10, and the final vec! with no semicolon is the normal return value
## Approach
1. Walk through the events, tracking a score and a counter
2. "W" adds 1 to the counter. "WD" and "NB" add 1 to the score. A number adds its value to the score
3. If the counter reaches 10, return early
4. Otherwise return [score, counter] after the loop
## Complexity
Time: O(n)  
Aux Space: O(1)
