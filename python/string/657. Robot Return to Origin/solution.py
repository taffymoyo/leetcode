class Solution:
    def judgeCircle(self, moves: str) -> bool:

        position = [0, 0]

        for move in moves:
            if move == "U":
                position[1] += 1
            elif move == "D":
                position[1] -= 1
            elif move == "R":
                position[0] += 1
            elif move == "L":
                position[0] -= 1

        return position == [0, 0]
