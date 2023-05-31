# Hogyan erdemes egy structot es az elemeit publikalni?
A structot azert kellett publikalni, hogy tudjam hasznalni az integracios tesztben.
A mezoket pedig azert, hogy ne kelljen konstruktort irni.

# Egy struct-ba torekedjunk-e arra, hogy referenciat tegyunk?
Amint elkezdtem &str-t irni, fel kellett scopeolni a structot. Emiatt azokat a strukturakat, amik ot tartalmazzak, azokat is fel kellett templatezni. A scope vegigvezetes fertoz.

# Hogy a kulonbozo modulban levo structjaim lassak egymast, a main.rs-be vegig kellett irni az osszes modult mod ...
Hogy az integracios tesztem leforduljon, kellett egy dummy lib.rs, amiben megint meg vannak ismetelve a modulok.

# Traitben a Result<&Sef, Box<dyn Error>>
Ez a Box-olas eleg kenyelmetlennek tunik.

# Unit teszteket irjak vagy inkabb integracios teszteket?
# Unit teszteket muszaj ugyanabba a fileba irni?

# Debug trait?
Ha nem implementalja az egyik field a debug traitet, akkor nem tudom a komponalt structra ratenni a debugot.

# van olyan assert amiben tudok assertalni pattern match stilusban? Result Ok-ot akarok tesztelni.
assert_matches! nincs stable-ben.
is_ok()-ra assert!-elek jelenleg.
