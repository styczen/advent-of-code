import os

# Part 1
with open('input_day_2', 'r') as f:
    input_data = f.readlines()

h = 0
d = 0

for cmd in input_data:
    split_cmd = cmd.split()
    direction, value = split_cmd[0], int(split_cmd[1])
    if direction == 'forward':
        h += value
    elif direction == 'down':
        d += value
    else:
        d -= value

print(h * d)

# Part 2
h = 0
d = 0
aim = 0

for cmd in input_data:
    split_cmd = cmd.split()
    direction, value = split_cmd[0], int(split_cmd[1])
    if direction == 'forward':
        h += value
        d += aim * value
    elif direction == 'down':
        aim += value
    else:
        aim -= value

print(h * d)
