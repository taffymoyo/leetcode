class Solution:
    def recoverOrder(self, order: List[int], friends: List[int]) -> List[int]:

        result = []
        friend_set = set(friends)

        for num in order:
            if num in friend_set:
                result.append(num)

        return result
