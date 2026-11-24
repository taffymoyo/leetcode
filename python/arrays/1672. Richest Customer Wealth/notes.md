# 1672. Richest Customer Wealth

## What I learned
- `sum()` calculates the total of an iterable.
- `max()` returns the largest value from an iterable.
- Generator expressions allow calculations without explicitly creating an intermediate list.

## Approach
1. Iterate through each customer's account.
2. Use `sum()` to calculate each customer's total wealth.
3. Use `max()` to find the highest total.
4. Return the maximum wealth.

## Complexity
- **Time:** \(O(m \times n)\) — iterate through all account balances.
- **Auxiliary Space:** \(O(1)\) — the generator expression does not create an additional list.
