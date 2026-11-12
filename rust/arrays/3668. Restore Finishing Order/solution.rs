use std::collections::HashSet; // set for faster lookup
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {

        let mut result = vec![];
        let set: HashSet<i32> = friends.into_iter().collect(); // move numbers into an iterator and then into a set through type annotation

        for num in &order {  // borrowing collection means every item inside is a reference
            if set.contains(num) { // contains wants &i32
                result.push(*num);
            }
        }
        result
        
    }
}
```
