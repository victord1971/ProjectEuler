#MOD = 1
MOD = 1123581313
K = 6

# Створюємо таблицю (K x K), заповнену нулями
A = [[0 for _ in range(K)] for _ in range(K)]

# Початкові умови A[0][0] = 0
if K > 1:
    A[0][1] = 1
    '''
if K > 2:
    A[0][2] = 1
for n in range(3, K):
    A[0][n] = (A[0][n-1] + A[0][n-2]) #% MOD  '''

# Заповнення таблиці згідно з рекурсіями
for m in range(1, K):
    for n in range(m+2):
        if n < K:
            if n == 0:
                A[m][n] = (A[m-1][n] + A[m-1][n+1]) #% MOD
            else:
                A[m][n] = (2 * A[m][n-1] + A[m-1][n-1]) #% MOD
                if n == m-1:
                    for x in range (m-1):
                        #t=0
                        A[x][n]=A[x+1][n-1] - A[x][n-1]

# Виведення таблиці для перевірки
for row in A:
    for a in row:
        print(a,end='\t')
    print()

# Обчислення суми S(K)
result = 0
for m in range(1, K):
    for n in range(1, K):
        result = (result + A[m][n]) 
#print(A)
print(result % MOD)

