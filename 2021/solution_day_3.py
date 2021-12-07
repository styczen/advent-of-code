import copy

# Part 1
with open('input_day_3', 'r') as f:
    data = [el.strip() for el in f.readlines()]

gamma_rate = ''
for digit in range(len(data[0])):
    count_ones = 0
    for n in data:
        count_ones += int(n[digit]) 
    if count_ones > len(data) / 2:
        gamma_rate += '1'
    else:
        gamma_rate += '0'

gamma_rate = int(gamma_rate, 2)
epsilon_rate = gamma_rate ^ int('1' * len(data[0]), 2)
print(gamma_rate * epsilon_rate)

# Part 2
def calc_rate(data, most_common):
    d0 = '1' if most_common else '0'
    d1 = '0' if most_common else '1'
    nums = copy.deepcopy(data)
    for dig in range(len(data[0])):
        if len(nums) == 1:
            break
        ones = len(list(filter(lambda el: el[dig] == '1', nums)))
        zeros = len(list(filter(lambda el: el[dig] == '0', nums)))
        if ones > zeros:
            nums = list(filter(lambda el: el[dig] == d0, nums))        
        elif ones < zeros:
            nums = list(filter(lambda el: el[dig] == d1, nums))        
        else:
            nums = list(filter(lambda el: el[dig] == d0, nums))        
    return int(nums[0], 2)

o2_rate = calc_rate(data, True)
co2_rate = calc_rate(data, False)

print(o2_rate * co2_rate)

