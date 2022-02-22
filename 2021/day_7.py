import numpy as np


with open('input_day_7', 'r') as f:
    data = f.read()

data = np.array([int(x) for x in data.split(',')])

# Part 1
min_fuel = 1000000 
for h in range(np.max(data)):
    fuel = np.sum(np.abs(data - h))
    min_fuel = min(fuel, min_fuel)

print(min_fuel)

# Part 2
min_fuel = 1000000000
for h in range(np.max(data) + 1):
    d = np.abs(data - h)
    # Just use equation for arithmetic progression 1, 2, 3, ..., d
    fuel = np.sum(d * (1 + d) // 2)
    min_fuel = min(fuel, min_fuel)

print(min_fuel)

