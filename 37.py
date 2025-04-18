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
	print(num)
print(a)