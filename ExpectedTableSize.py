#!/usr/bin/python

import sys

N = int(sys.argv[1]) - 1 # Subtract ourselves
GROUP_SIZE = 8

hidden = 0
for i in range(0, 512):
    nodes_with_bucket_index = N * (0.5 ** (i + 1))
    if nodes_with_bucket_index <= GROUP_SIZE:
        break
    hidden += nodes_with_bucket_index - GROUP_SIZE

print("Expected number of routing table entries: " + str(int(N - hidden)))
