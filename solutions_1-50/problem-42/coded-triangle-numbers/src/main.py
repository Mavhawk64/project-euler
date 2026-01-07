
def get_nth_triangle_number(n):
    return n * (n + 1) // 2

def get_n(t):
    return int(((8 * t + 1) ** (1/2) - 1) / 2)


with open("/home/maverick/repos/project-euler/solutions_1-50/problem-42/coded-triangle-numbers/src/words.txt", "r", encoding="utf-8") as f:
    words = [i.replace('"', '') for i in f.read().split('\n')[0].split(',')]

cnt = 0

for i in words:
    t = sum([ord(x) - 64 for x in i])
    if t == get_nth_triangle_number(get_n(t)):
        print(i)
        cnt += 1

print(cnt)
