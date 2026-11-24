class Solution:
    def triangleType(self, nums: List[int]) -> str:

        res = set(nums)

        if (nums[0] + nums[1] <= nums[2] or
        nums[0] + nums[2] <= nums[1] or
        nums[1] + nums[2] <= nums[0]):
            return "none"

        if len(res) == 1:
            return "equilateral"
        if len(res) == 2:
                return "isosceles"
        if len(res) == 3:
                return "scalene"
        return "none"
        
