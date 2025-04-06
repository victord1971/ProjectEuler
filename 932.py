#import
T4=0
T16=5131
for num in range(11,200):
#for num in range(10_000,10_150):
    snum=str(num)
    for digit in range(1,len(snum)):
        print(num, snum[0:digit], snum[digit:])
    print()

