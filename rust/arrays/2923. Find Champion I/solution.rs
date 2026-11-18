impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {

        let mut winners: bool = true;
        
        for (i, g) in grid.iter().enumerate() {
            winners = true;
            for (j, num) in g.iter().enumerate() {
                if i != j && * num == 0 {
                        winners = false;
                        break;
                }
            }
            if winners {
                return i as i32;
            }
        }
        -1
    }
}
