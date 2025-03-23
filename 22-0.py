# Open the names.txt file and read the contents
with open('/home/vic/Common/Python/ProjectEuler/names.txt', 'r') as file:
    content = file.read()
names = content.replace('"', '').split(',')
# Strip any extra spaces from each name
names = [name.strip() for name in names]
names.sort()
print(ord('A')-64)
