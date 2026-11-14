# 2303. Calculate Amount Paid in Taxes

## What I learned
* Iterating over `&brackets` borrows the vector instead of taking ownership
* `.min()` returns the smaller of two values, useful for calculating taxable income within a bracket
* Use `as f64` to convert integers into floating-point numbers for decimal calculations
* Divide the tax percentage by `100.0` to convert it into a decimal rate
* Track `prev_upper` to calculate only the income belonging to the current tax bracket
* `break` exits the loop once the remaining income is no longer taxable

## Approach
1. Initialise `tax_amount` to `0.0` and `prev_upper` to `0`
2. Iterate through each tax bracket
3. Check whether income exceeds the previous bracket's upper limit
4. Calculate the taxable amount using `income.min(bracket[0]) - prev_upper`
5. Multiply the taxable amount by the bracket's tax rate and add it to the total
6. Update `prev_upper` to the current bracket's upper limit
7. Break if no remaining income falls within the current bracket
8. Return the total tax

## Complexity
Time: O(n) — iterate through the tax brackets  
Aux Space: O(1) — constant extra space
