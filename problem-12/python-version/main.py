from math import sqrt


def main():
    iter = 1
    i = 1
    f = get_factors(i)
    bf = 1
    while len(f) < 500:
        iter += 1
        i += iter
        f = get_factors(i)
        if len(f) > bf:
            bf = len(f)
            print("New biggest: {} with {} factors: {}", i, len(f), f)
    print("{} has {} factors.\nHere they are listed:\n{}", i, len(f), f)


def get_factors(n):
    v = []
    for i in range(1, int(sqrt(n)) + 1):
        if n % i == 0:
            v.append(i)
            v.append(n // i)
    v.sort()
    return v


if __name__ == "__main__":
    main()
