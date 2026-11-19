impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        
        let mut children: i32 = children;
        let mut money: i32 = money;
        let mut count: i32 = 0;

        if money < children {
            return -1;
        }

        money -= children;

        let mut eight_childs = money / 7; // evaluates to floor deviison
        let mut remainder = money % 7;

        if eight_childs > children || (eight_childs == children && remainder > 0) {
            return children -1;
        } 

        if children - eight_childs == 1 && remainder == 3 { // last child forced to total to 4
            return eight_childs - 1;
        }

        eight_childs
    }
}
