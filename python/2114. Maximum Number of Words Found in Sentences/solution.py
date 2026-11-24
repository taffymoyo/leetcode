class Solution:
    def mostWordsFound(self, sentences: list[str]) -> int:

        maximum = 0

        for sen in sentences:
            maximum = max(len(sen.split()), maximum)
        
        return maximum
