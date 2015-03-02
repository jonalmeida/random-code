#!/usr/bin/py

import sys;


lowest_recorded = sys.maxint
result = []

def lonelyinteger(a):
    global lowest_recorded
    global result;
    answer = 0
    a.sort()
    for x in range(0, len(b)-1):
        diff_value = abs(b[x] - b[x+1])
        #print "for " + str(b[x]) + " and " + str(b[x+1]) + " the diff_value is: " + str(diff_value)
        #print "lowest_recorded: " + str(lowest_recorded)
        if diff_value < lowest_recorded:
            result = []
            result.append(b[x])
            result.append(b[x+1])
            lowest_recorded = diff_value
        elif diff_value == lowest_recorded:
            result.append(b[x])
            result.append(b[x+1])
    return ' '.join(map(str, result))


if __name__ == '__main__':
    a = input()
    b = map(int, raw_input().strip().split(" "))
    #a = 12
    #b = [-20, -3916237, -357920, -3620601, 7374819, -7330761, 30, 6246457, -6461594, 266854, -520, -470]
    print lonelyinteger(b)

