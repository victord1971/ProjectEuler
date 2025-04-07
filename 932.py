#import
T4=0
T16=5131
for num in range(11,10_000_000_000_000_000):
#for num in range(10_000,100_000):
    snum=str(num)
    for digit in range(1,len(snum)):
        if snum[digit:digit+1] != '0':
            te1=snum[0:digit]
            te2=snum[digit:]
            #print(num, snum[0:digit], snum[digit:])
            #print(num, te1, te2)
            if (int(te1)+int(te2))**2 == num:
                T4+=num
                print('             ',num, te1, te2)
    #print()
print(T4)
