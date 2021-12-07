with open('input_day_4', 'r') as f:
    drawn_nums = f.readline().strip().split(',')
    drawn_nums = [int(n) for n in drawn_nums]
    print(drawn_nums)
    print('A', f.readline())
    print('B', f.readline())

