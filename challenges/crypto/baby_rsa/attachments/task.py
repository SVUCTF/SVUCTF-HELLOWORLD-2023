from Crypto.Util.number import *
from secret import flag

e = 65537
m = bytes_to_long(flag.encode())
p, q = getPrime(1024), getPrime(1024)
assert p > q
n = p*q
c = pow(m, e, n)
print(c)
print(n)
print(p-q)
# 10756710205748611384521688249412262978891051674402229623415380014251730751904569939120028982914842328041361408532249623249079526546193656744759450750407428092225739012672357833366967266816932330391904969472014866936674797534358725190755455761365128730630492034560493436145174732884107653037527176947479768505830481947003710722053078523547649655418323071360911828571417208067496702709363600424022303681457599009476227923445048580047659478431351160857155504187696870155468903806429133785952756283022656773650952543940775762413319496189075962678266845507251961321996743500843027116959060185852297612725153033030184581506
# 12416761377421529271201014438297952616813863815839577814720736353146837999879900552360138896254812646860094233797682761630909343680994025755324527515699802994468746410776553400519881874126308834513964753223848852009336573455678458412940004221087564838886589069163241776612237771498954996924724128871380187540383878210649701026098171999297301375345739229593863438700570965076033601996943529300850173276773095789634518862597955407830745300258008315247457499373044241277311966619358878032303717644973713594989575828565689225317841697555727167495197675432429064277213861644371317624376161180612580989820255418798411360873
# 29991006358655151128453519577456973361885941223137013808304137174063429807597041878938588259623798491625198324717082746140826552683594382913716870816725326409012446193287108036231237147328479402942011455483920411110272987972475049046196663941124684342049507282802918829741425782759503502760187131358912000888