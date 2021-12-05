import os

with open('input_1', 'r') as f:
    input_data = f.readlines()

input_data = [int(el.strip()) for el in input_data]

cnt = 0
for i in range(1, len(input_data)):
    if input_data[i] > input_data[i - 1]:
        cnt += 1

print('Answer:', cnt)

