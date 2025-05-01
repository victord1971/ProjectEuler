from functools import cache

MOD = 1_000_000_007

# Числа Фібоначчі до F_50
def fibonacci_up_to_index(k):
    fib = [0, 1]
    for _ in range(2, k + 2):
        fib.append((fib[-1] + fib[-2]) % MOD)
    return fib[1:k + 1]  # без 0

fib = fibonacci_up_to_index(50)

# Рекурсивна функція з кешуванням для A(m, n)
@cache
def A(m, n):
    if m == 0 and n == 0:
        return 0
    if m == 0 and n == 1:
        return 1
    if m == 0:
        return (A(0, n - 1) + A(0, n - 2)) % MOD
    if n == 0:
        return (A(m - 1, 1) + A(m - 1, 0)) % MOD
    return (2 * A(m, n - 1) + A(m - 1, n - 1)) % MOD

# Обчислення S(50)
S_50 = sum(A(m, n) for m in fib[1:] for n in fib[1:]) % MOD
print(S_50)
