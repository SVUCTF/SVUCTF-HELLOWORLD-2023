from z3 import *

v = Real('v')
w = Real('w')
x = Real('x')
y = Real('y')
z = Real('z')

s = Solver()

s.add(v * 24 + w * -32 + x * 98 + y * 55 + z * 90 == 153908)
s.add(v * 123 + w * -332 + x * 68 + y * 67 + z * 32 == -63551)
s.add(v * 266 + w * -34 + x * 44 + y * 8 + z * 32 == 190984)
s.add(v * 454 + w * -302 + x * 51 + y * 65 + z * 5 == 124630)
s.add(v * 321 + w * -321 + x * 938 + y * 565 + z * 970 == 1543640)

if s.check() == sat:
    result = s.model()
    print (result)
else:
    print (b'no result')

