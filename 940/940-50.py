# Функція для обчислення чисел Фібоначчі до певного k
def fibonacci_up_to(k):
    fib = [0, 1]
    for _ in range(2, k + 2):
        fib.append(fib[-1] + fib[-2])
    return fib[1:k + 1]  # без нуля

# Перерахуємо A(m, n) таблично, використовуючи точні рекурсії з задачі
def compute_A_table(max_m, max_n):
    A = [[0] * (max_n + 2) for _ in range(max_m + 2)]
    A[0][1] = 1  # базовий випадок

    for m in range(max_m + 1):
        for n in range(max_n + 1):
            if m + 1 <= max_m + 1:
                A[m + 1][n] += A[m][n + 1] + A[m][n]
            if m + 1 <= max_m + 1 and n + 1 <= max_n + 1:
                A[m + 1][n + 1] = 2 * A[m + 1][n] + A[m][n]

    return A

# Отримаємо числа Фібоначчі до f_50
fib = fibonacci_up_to(50)

# Нам потрібно максимально f_50 за m, n
max_fib = fib[-1]
A_table = compute_A_table(max_fib, max_fib)

# Обчислимо S(50)
S_50 = sum(A_table[m][n] for m in fib[1:] for n in fib[1:])
print(S_50)
