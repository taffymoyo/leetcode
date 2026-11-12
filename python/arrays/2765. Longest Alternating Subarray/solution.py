class Solution:
    def alternatingSubarray(self, nums: List[int]) -> int:

        longest = -1
        for i, num in enumerate(nums):
            first, second = True, False
            count = -1

            for j in range(i + 1, len(nums)):

                if first:
                    if nums[i] - nums[j] != -1:
                        longest = max(count, longest)
                        break
                    second = True
                    first = False
                    
                elif second:
                    if nums[i] - nums[j] != 0:
                        longest = max(count, longest)
                        break
                    second = False
                    first = True

                count += 1 if count != -1 else 3
                longest = max(count, longest)
  
        return longest