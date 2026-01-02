import math

def find_permutation(digits, n):
    # Adjust n to be 0-indexed (the 1,000,000th is index 999,999)
    n -= 1
    result = []

    # We use a copy of the digits because we will pop elements out
    available_digits = list(digits)

    for i in range(len(digits) - 1, -1, -1):
        # How many permutations exist for the remaining slots?
        fact = math.factorial(i)

        # Determine which digit belongs in the current position
        index = n // fact
        result.append(available_digits.pop(index))

        # Update n for the next position
        n %= fact

    return "".join(map(str, result))

# Usage for your specific problem:
digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
print(find_permutation(digits, 1000000))
