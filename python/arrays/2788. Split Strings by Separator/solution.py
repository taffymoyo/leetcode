class Solution:
    def splitWordsBySeparator(self, words: List[str], separator: str) -> List[str]:

        words = [word.split(separator) for word in words]
        flat_list = [word for sublist in words for word in sublist if word]  # nested comprehension
        return flat_list
