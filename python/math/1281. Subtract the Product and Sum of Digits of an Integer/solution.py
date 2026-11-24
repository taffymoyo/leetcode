class Solution:
    def subtractProductAndSum(self, n: int) -> int:

        product, summ = 1, 0

        for digit in str(n):
            product *= int(digit)
            summ += int(digit)

        return product - summ
        
