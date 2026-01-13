from mpmath import cos, cosh, floor, mp, pi, sqrt

# Set high precision (100 decimal places should be enough)
mp.dps = 100


def is_perfect_square(n):
    sqrt_n = int(sqrt(abs(n)))
    return sqrt_n * sqrt_n == abs(n)


mini = {"n": -1, "d": mp.mpf("999")}

# Positive case: cos(pi * sqrt(n))
for n in range(1, 10**3 + 1):
    if is_perfect_square(n):
        continue  # skip perfect squares

    value = cos(pi * sqrt(n))
    nearest = floor(value + mp.mpf("0.5"))  # round to nearest integer
    d = abs(nearest - value)

    if d < mini["d"]:
        mini["n"] = n
        mini["d"] = d
        mini["value"] = value
        mini["nearest"] = nearest

# Negative case: cos(pi*i*sqrt(|n|)) = cosh(pi*sqrt(|n|))
for n in range(1, 10**3 + 1):
    if is_perfect_square(n):
        continue  # skip perfect squares

    value = cosh(pi * sqrt(n))
    nearest = floor(value + mp.mpf("0.5"))
    d = abs(nearest - value)

    if d < mini["d"]:
        mini["n"] = -n
        mini["d"] = d
        mini["value"] = value
        mini["nearest"] = nearest

print(mini["n"])
