uin = ""
out = ''
ind = int(input('indentation level: '))
while uin != "stop":
	uin = input()
	if uin != 'stop':
		ar = [ord(c) for c in uin]
		out += '\n'
		out += ' '*ind
		out += str(ar)

with open('/home/michael/tempout.txt', 'w') as f:
	f.write(out)