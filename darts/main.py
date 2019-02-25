# https://www.codingame.com/ide/puzzle/darts

import sys
import collections

def get_score(point):
    r = size/2
    if point[0] + point[1] <= r:
        return 15
    if point[0]*point[0] + point[1]*point[1] <= r*r:
        return 10
    if point[0] <= r and point[1] <= r:
        return 5
    return 0

size = int(input())
scores = collections.OrderedDict()
for i in range(int(input())):
    scores[input()] = 0

for i in range(int(input())):
    throw_name, throw_x, throw_y = input().split()
    scores[throw_name] += get_score((abs(int(throw_x)), abs(int(throw_y))))

final = sorted(scores.items(), key = lambda x: x[1], reverse=True)
print("\n".join(["{} {}".format(s[0], s[1]) for s in final]))
