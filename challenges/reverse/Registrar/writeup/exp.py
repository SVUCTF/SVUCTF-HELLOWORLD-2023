str=['d','d','2','9','4','0','c','0','4','4','6','2','b','4','d','d','7','c','4','5','0','5','2','8','8','3','5','c','c','a','1','5']

str[2] = chr(ord(str[2]) + ord(str[3]) - 50)
str[4] = chr(ord(str[2]) + ord(str[5]) - 48)
str[30] = chr(ord(str[31]) + ord(str[9]) - 48)
str[14] = chr(ord(str[27]) + ord(str[28]) - 97)

for i in range(16):
    s = str[31-i]
    str[31-i] = str[i]
    str[i] = s

for i in str:
    print (i,end="")

