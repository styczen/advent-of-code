import os

# Part 1
with open('input_day_1', 'r') as f:
    input_data = f.readlines()

input_data = [int(el.strip()) for el in input_data]

cnt = 0
for i in range(1, len(input_data)):
    if input_data[i] > input_data[i - 1]:
        cnt += 1

print('Answer:', cnt)

# Part 2
cnt = 0
for i in range(1, len(input_data) - 2):
    first_sum = input_data[i - 1] + input_data[i] + input_data[i + 1]
    second_sum = input_data[i] + input_data[i + 1] + input_data[i + 2]
    if second_sum > first_sum:
        cnt += 1

print('Answer:', cnt)

