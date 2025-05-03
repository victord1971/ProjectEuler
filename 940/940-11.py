MOD = 1123581313

# Compute Fibonacci numbers up to F_50
fib = [0, 1]
for _ in range(2, 51):
    fib.append(fib[-1] + fib[-2])

# Unique values from F_2 to F_50
fibs = sorted(set(fib[2:]))

# Memoization cache
cache = {}

def compute_A(m, n):
    if (m, n) in cache:
        return cache[(m, n)]

    if m == 0:
        val = 1 if n == 1 else 0
    elif n == 0:
        val = compute_A(m - 1, 1)
    else:
        val = (2 * compute_A(m, n - 1) + compute_A(m - 1, n - 1)) % MOD

    cache[(m, n)] = val
    return val

# Compute S(50)
S = 0
for m in fibs:
    for n in fibs:
        S = (S + compute_A(m, n)) % MOD

print(S)
