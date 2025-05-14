MOD = 1123581313  # Можна змінити або прибрати

def compute_S(K, mod=MOD):
    # Створюємо таблицю A (K x K)
    A = [[0] * K for _ in range(K)]

    # Початковий рядок A[0][n]
    if K > 1:
        A[0][1] = 1
    if K > 2:
        A[0][2] = 1
    for n in range(3, K):
        A[0][n] = (A[0][n-2] + A[0][n-3]) % mod

    # Заповнення всієї таблиці A
    for m in range(1, K):
        for n in range(K):
            if n == 0:
                A[m][0] = (A[m-1][0] + A[m-1][1]) % mod
            else:
                A[m][n] = (2 * A[m][n-1] + A[m-1][n-1]) % mod

    # Виведення таблиці для перевірки
    for row in A:
        print(row)

    # Обчислення суми S(K)
    result = 0
    for m in range(1, K):
        for n in range(m+1, K):  # лише коли m < n
            result = (result + A[m][n]) % mod

    return result

# Приклад для K = 5
S_5 = compute_S(6)  # Кількість елементів таблиці (для K=5, це 6)
print(f"S(5) = {S_5}")
