#MOD = 1
MOD = 1123581313
k=50
a=7
f=[0,1]
for x in range(2,k+1):
    f.append(f[x-1]+f[x-2])
print(f)
K = f[k]+1
print(K)
f.pop(0); f.pop(0)
print(f)

# Створюємо таблицю (k+2 x k+2), заповнену нулями
A = [[0 for _ in range(k+2)] for _ in range(k+2)]

# Початкові умови 
A[0][1]=1; A[0][2]=1; A[0][3]=4; A[0][4]=7; A[0][5]=19; A[1][0]=1
A[1][1]=2; A[1][2]=5; A[1][3]=11; A[1][4]=26; A[1][5]=59; A[2][0]=3
A[2][1]=7; A[2][2]=16; A[2][3]=37; A[2][4]=85; A[2][5]=196; A[3][0]=10
A[3][1]=23; A[3][2]=53; A[3][3]=12; A[3][4]=281; A[3][5]=647; A[4][0]=109
A[4][1]=251; A[4][2]=578; A[4][3]=1331; A[4][4]=3065; A[4][5]=7058; 

# Виведення таблиці для перевірки
for row in range(a):
    for col in range(a):
        print(A[row][col],end='\t')
    print() 

#print(A)

# Обчислення суми S(K)
result = 0
for m in f:
    for n in f:
        #if m>1 and n>1:
        result = (result + A[m][n]) 
#print(A)
print(result) #% MOD)
