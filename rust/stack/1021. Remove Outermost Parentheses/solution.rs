impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        
        let mut result: Vec<char> = vec![];
        let mut stack: Vec<char> = vec![];

        for par in s.chars() {

            if par == '(' { // single quotes is char type
                if stack.len() != 0 { 
                    result.push(par); // only add non outer
                } 
                stack.push(par);
                
            } else {
                stack.pop();
                if stack.len() != 0 { 
                    result.push(par);
                }
            }           
        }
      
        let res: String = result.iter().collect();
        res
    }
}
