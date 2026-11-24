from functools import reduce
from operator import xor

class Solution:
    def xorOperation(self, n: int, start: int) -> int:

        arr = [start + 2 * i for i in range(n)]
        return reduce(xor, arr, 0) 
        # reduce applys a function to an iterable and combines into a single result
        # third argument is a initial val
        
