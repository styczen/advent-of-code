# Part 1
cnt = 0

with open('input_day_8', 'r') as f:
    l = f.readline()
    while l:
        s, d = l.split('|')
        d = d.split()
        for x in d:
            if len(x) < 5 or len(x) == 7:
                cnt += 1
        l = f.readline()

print(cnt)

# Part 2
# 0 - abcefg
# 1 - cf
# 2 - acdeg
# 3 - acdfg
# 4 - bcdf
# 5 - abdfg
# 6 - abdefg
# 7 - acf
# 8 - abcdefg
# 9 - abcdfg

cnt = 0

with open('input_day_8', 'r') as f:
    l = f.readline()
    while l:
        s, d = l.split('|')
        s = s.split()
        d = d.split()
        print(s)
        print(d)
        print('---')
        

        l = f.readline()

print(cnt)
