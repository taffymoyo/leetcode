impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {

        let mut tax_amount: f64 = 0.0;
        let mut prev_upper: i32 = 0;

        for bracket in &brackets {
            if income > prev_upper {
                tax_amount += (income.min(bracket[0]) - prev_upper) as f64 * (bracket[1] as f64 / 100.0);
                prev_upper = bracket[0];
            } else {
                break;
            }
        }
        tax_amount
    }
}
