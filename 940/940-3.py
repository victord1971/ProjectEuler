# Функція для обчислення чисел Фібоначчі до певного k
def fibonacci_up_to(k):
    fib = [0, 1]
    for _ in range(2, k + 2):
        fib.append(fib[-1] + fib[-2])
    return fib[1:k + 1]  # без нуля

# Використаємо memoization для рекурсивного A(m, n)
from functools import lru_cache

@lru_cache(maxsize=None)
def A(m, n):
    if m == 0 and n == 0:
        return 0
    if m == 0 and n == 1:
        return 1
    if m == 0:
        return A(0, n - 1) + A(0, n - 2)  # Фібоначчі (лише в основі)
    if n == 0:
        return A(m - 1, 1) + A(m - 1, 0)
    return 2 * A(m, n - 1) + A(m - 1, n - 1)

# Тепер порахуємо S(5)
fib = fibonacci_up_to(5)
S_5 = sum(A(m, n) for m in fib[1:] for n in fib[1:])  # від f_2 до f_5
print(S_5)
