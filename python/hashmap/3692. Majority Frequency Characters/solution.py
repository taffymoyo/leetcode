from collections import Counter

class Solution:
    def majorityFrequencyGroup(self, s: str) -> str:
        
        counter = Counter(s)
        result = ""

        seen_sizes = {}
        for key, val in counter.items():
            if val not in seen_sizes:
                seen_sizes[val] = 1
            else:
                seen_sizes[val] += 1
        
        largest_val = (0, 0)
        for key, val in seen_sizes.items():
            if val > largest_val[1]:
                largest_val = (key, val)
            elif val == largest_val[1]:
                if key > largest_val[0]:
                    largest_val = (key, val)
        
        for key in counter:
            if counter[key] == largest_val[0]:
                result += key
        
        return result
