a=[2]
for num in range(3,100_000):
	t=0
	for x in a:
		if num%x==0:
			t=1
			break
	if t>0:
		continue
	a.append(num)
	#print(num)
print(a)
for num in a:
    snum=str(num)
    for digit in range(1,len(snum)):
        if snum[digit:digit+1] != '0':
            te1=snum[0:digit]
            te2=snum[digit:]
            print(num, te1, te2)
	#print(' ')

