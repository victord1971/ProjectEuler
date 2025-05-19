from functools import lru_cache

# W(k): допустимі слова довжини k
def W(k):
    return 4 * (3 ** (k - 1))

# B(n): кількість розбиттів довжини n на допустимі слова (довжини 1–4)
@lru_cache(maxsize=None)
def B(n):
    if n == 0:
        return 1
    total = 0
    for k in range(1, 5):
        if n >= k:
            total += W(k) * B(n - k)
    return total

# Побудова таблиці A[m][n] для 0 ≤ m, n ≤ k
def build_A(k):
    A = [[0] * (k + 1) for _ in range(k + 1)]
    for m in range(k + 1):
        for n in range(k + 1):
            if m == 0:
                A[m][n] = B(n)
            elif n == 0:
                A[m][n] = B(m)
            else:
                A[m][n] = A[m - 1][n] + A[m][n - 1]
    return A

# Обчислення S(k)
def S(k):
    A = build_A(k)
    total = 0
    for m in range(k):
        for n in range(m + 1, k + 1):
            total += A[m][n]
    return total

# Перевірка
print("S(5) =", S(5))    # Має бути 10396
