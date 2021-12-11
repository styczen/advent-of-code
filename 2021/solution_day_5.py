import numpy as np


size_x, size_y = 0, 0
coords = []
with open('input_day_5', 'r') as f:
    line = f.readline()
    while line:
        start, end = line.split('->')
        x1, y1 = int(start.split(',')[0]), int(start.split(',')[1])
        x2, y2 = int(end.split(',')[0]), int(end.split(',')[1])
        coords.append([x1, y1, x2, y2])
        size_x = max(x1, x2, size_x)
        size_y = max(y1, y2, size_y)

        line = f.readline()

# Part 1
m = np.zeros((size_x + 1, size_y + 1))
for coord in coords:
    if coord[0] == coord[2]:
        m[coord[0], min(coord[1], coord[3]):max(coord[1], coord[3]) + 1] += 1
    elif coord[1] == coord[3]:
        m[min(coord[0], coord[2]):max(coord[0], coord[2]) + 1, coord[1]] += 1

result = (m >= 2).sum()
print(result)

# Part 2
m = np.zeros((size_x + 1, size_y + 1))
for coord in coords:
    if coord[0] == coord[2]:
        m[coord[0], min(coord[1], coord[3]):max(coord[1], coord[3]) + 1] += 1
    elif coord[1] == coord[3]:
        m[min(coord[0], coord[2]):max(coord[0], coord[2]) + 1, coord[1]] += 1
    elif abs(coord[0] - coord[2]) == abs(coord[1] - coord[3]):
        dx = coord[2] - coord[0]
        dy = coord[3] - coord[1]
        steps = abs(dx)
        for i in range(steps + 1):
            x = coord[0] + i * dx // steps
            y = coord[1] + i * dy // steps
            m[x, y] += 1

result = (m >= 2).sum()
print(result)
