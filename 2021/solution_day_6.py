from collections import defaultdict
from typing import Counter


def sim_step(fish):
    new_fish = defaultdict(int)
    for age, count in fish.items():
        if age == 0:
            new_fish[6] += count
            new_fish[8] += count
        else:
            new_fish[age - 1] += count
    return new_fish


with open('input_day_6', 'r') as f:
    data = f.read()

data = [int(x) for x in data.split(',')]
fish = defaultdict(int, Counter(data))

for _ in range(80):
    fish = sim_step(fish)
print(sum(fish.values()))

for _ in range(256 - 80):
    fish = sim_step(fish)
print(sum(fish.values()))

