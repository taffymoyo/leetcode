# 2591. Distribute Money to Maximum Children
## What I learned
* Give every child 1 dollar first, which covers the "at least 1 dollar each" rule. Each child then needs 7 more dollars to reach 8, so money / 7 is how many children can get exactly 8
* If money < children, there's no valid split, so return -1
* Too much money (eight_childs > children, or eight_childs == children with a remainder) means everyone can't get 8, because all the money must be spent. The extra goes to one child, so at most children - 1 get exactly 8
* The 4 dollar trap: if everyone but one child gets 8 and the last child would be left with exactly 4 (remainder == 3 after the 1 dollar head start), that child can't be given 4, so one fewer child gets 8
* The last line is the function's return value, with no semicolon
## Approach
1. If money < children, return -1
2. Subtract 1 dollar per child, then compute how many children can get 7 more (eight_childs) and what's left (remainder)
3. If there's more money than all children can take as 8, or exactly enough for all but with a leftover, return children - 1
4. If exactly one child is left over and would end up with 4 dollars, return eight_childs - 1
5. Otherwise return eight_childs
## Complexity
Time: O(1)
Aux Space: O(1)
