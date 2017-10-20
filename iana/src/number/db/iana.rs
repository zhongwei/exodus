#[feature(i128_type)]

pub static IPV4_NUMBERS: [(u32, u32, u8, u8); 115] = [
    (0, 16777216, 237, 7), (16777216, 16777216, 237, 2), (33554432, 16777216, 237, 10), (50331648, 33554432, 237, 3), (83886080, 16777216, 237, 10), 
    (100663296, 67108864, 237, 3), (167772160, 16777216, 237, 7), (184549376, 50331648, 237, 3), (234881024, 16777216, 237, 2), (251658240, 167772160, 237, 3), (419430400, 16777216, 237, 10), 
    (436207616, 16777216, 237, 3), (452984832, 16777216, 237, 2), (469762048, 50331648, 237, 3), (520093696, 16777216, 237, 10), (536870912, 67108864, 237, 3), (603979776, 16777216, 237, 2), 
    (620756992, 16777216, 237, 10), (637534208, 16777216, 237, 3), (654311424, 16777216, 237, 2), (671088640, 16777216, 237, 3), (687865856, 16777216, 237, 0), (704643072, 33554432, 237, 2), 
    (738197504, 33554432, 237, 3), (771751936, 16777216, 237, 10), (788529152, 33554432, 237, 3), (822083584, 16777216, 237, 2), (838860800, 16777216, 237, 3), (855638016, 16777216, 237, 10), 
    (872415232, 100663296, 237, 3), (973078528, 67108864, 237, 2), (1040187392, 16777216, 237, 10), (1056964608, 234881024, 237, 3), (1291845632, 318767104, 237, 10), (1610612736, 71303168, 237, 3), 
    (1681915904, 4194304, 237, 7), (1686110208, 8388608, 237, 3), (1694498816, 16777216, 237, 2), (1711276032, 16777216, 237, 0), (1728053248, 16777216, 237, 2), (1744830464, 16777216, 237, 3), 
    (1761607680, 16777216, 237, 0), (1778384896, 16777216, 237, 2), (1795162112, 33554432, 237, 3), (1828716544, 16777216, 237, 10), (1845493760, 285212672, 237, 2), (2130706432, 16777216, 237, 7), 
    (2147483648, 83886080, 237, 3), (2231369728, 16777216, 237, 2), (2248146944, 117440512, 237, 3), (2365587456, 16777216, 237, 10), (2382364672, 50331648, 237, 3), (2432696320, 16777216, 237, 10), 
    (2449473536, 67108864, 237, 3), (2516582400, 16777216, 237, 2), (2533359616, 16777216, 237, 10), (2550136832, 16777216, 237, 3), (2566914048, 16777216, 237, 2), (2583691264, 16777216, 237, 0), 
    (2600468480, 134217728, 237, 3), (2734686208, 16777216, 237, 2), (2751463424, 100532224, 237, 3), (2851995648, 65536, 237, 7), (2852061184, 16842752, 237, 3), (2868903936, 16777216, 237, 2), 
    (2885681152, 1048576, 237, 3), (2886729728, 1048576, 237, 7), (2887778304, 48234496, 237, 3), (2936012800, 16777216, 237, 2), (2952790016, 16777216, 237, 10), (2969567232, 16777216, 237, 8), 
    (2986344448, 16777216, 237, 10), (3003121664, 16777216, 237, 8), (3019898880, 16777216, 237, 2), (3036676096, 16777216, 237, 8), (3053453312, 33554432, 237, 2), (3087007744, 16777216, 237, 3), 
    (3103784960, 16777216, 237, 10), (3120562176, 33554432, 237, 8), (3154116608, 16777216, 237, 10), (3170893824, 50331648, 237, 8), (3221225472, 256, 237, 7), (3221225728, 256, 237, 3), 
    (3221225984, 256, 237, 7), (3221226240, 813568, 237, 3), (3222042368, 1264896, 237, 3), (3223307520, 1375232, 237, 3), (3224683008, 939264, 237, 3), (3225622528, 106752, 237, 3), 
    (3225729536, 132608, 237, 3), (3225864704, 1153280, 237, 3), (3227017984, 256, 237, 7), (3227018240, 387328, 237, 3), (3227406080, 4829440, 237, 3), (3232235520, 65536, 237, 7), 
    (3232301056, 5701632, 237, 3), (3238002688, 50331648, 237, 10), (3288334336, 33554432, 237, 0), (3321888768, 1179648, 237, 3), (3323068416, 131072, 237, 7), (3323199488, 2057216, 237, 3), 
    (3325256704, 256, 237, 7), (3325256960, 2998528, 237, 3), (3328255744, 27187456, 237, 3), (3355443200, 33554432, 237, 8), (3388997632, 16806144, 237, 2), (3405803776, 256, 237, 7), 
    (3405804032, 16748032, 237, 2), (3422552064, 100663296, 237, 3), (3523215360, 33554432, 237, 2), (3556769792, 33554432, 237, 10), (3590324224, 50331648, 237, 3), (3640655872, 16777216, 237, 10), 
    (3657433088, 100663296, 237, 2), (3758096384, 536870912, 237, 7), 
];

pub static IPV6_NUMBERS: [(u128, u128, u8, u8); 56] = [
    (0, 1329227995784915872903807060280344575, 237, 7), (1329227995784915872903807060280344576, 2658455991569831745807614120560689151, 237, 7), (2658455991569831745807614120560689152, 5316911983139663491615228241121378303, 237, 7), 
    (5316911983139663491615228241121378304, 10633823966279326983230456482242756607, 237, 7), (10633823966279326983230456482242756608, 21267647932558653966460912964485513215, 237, 7), (21267647932558653966460912964485513216, 42535295865117307932921825928971026431, 237, 7), (42535295865117307932921825928971026432, 42540488161975842760550356425300246527, 237, 7), 
    (42540528726795050063891204319802818560, 42540569291614257367232052214305390591, 237, 2), (42540569291614257367232052214305390592, 42540609856433464670572900108807962623, 237, 3), (42540609856433464670572900108807962624, 42540650421252671973913748003310534655, 237, 10), (42540650421252671973913748003310534656, 42540690986071879277254595897813106687, 237, 10), 
    (42540690986071879277254595897813106688, 42540731550891086580595443792315678719, 237, 10), (42540731550891086580595443792315678720, 42540772115710293883936291686818250751, 237, 2), (42540772115710293883936291686818250752, 42540812680529501187277139581320822783, 237, 2), (42540853245348708490617987475823394816, 42540893810167915793958835370325966847, 237, 8), 
    (42540893810167915793958835370325966848, 42540934374987123097299683264828538879, 237, 10), (42540934374987123097299683264828538880, 42540974939806330400640531159331110911, 237, 10), (42540974939806330400640531159331110912, 42541015504625537703981379053833682943, 237, 3), (42541015504625537703981379053833682944, 42541056069444745007322226948336254975, 237, 10), 
    (42541056069444745007322226948336254976, 42541137199083159614003922737341399039, 237, 10), (42541137199083159614003922737341399040, 42541461717636818040730705893361975295, 237, 10), (42541461717636818040730705893361975296, 42541623976913647254094097471372263423, 237, 10), (42541623976913647254094097471372263424, 42541705106552061860775793260377407487, 237, 10), 
    (42541786236190476467457489049382551552, 42541826801009683770798336943885123583, 237, 10), (42541826801009683770798336943885123584, 42541867365828891074139184838387695615, 237, 0), (42541867365828891074139184838387695616, 42541907930648098377480032732890267647, 237, 2), (42541907930648098377480032732890267648, 42541948495467305680820880627392839679, 237, 10), 
    (42541948495467305680820880627392839680, 42541989060286512984161728521895411711, 237, 3), (42541989060286512984161728521895411712, 42542029625105720287502576416397983743, 237, 10), (42542029625105720287502576416397983744, 42542070189924927590843424310900555775, 237, 10), (42542110754744134894184272205403127808, 42542435273297793320911055361423704063, 237, 10), 
    (42543084310405110174364621673464856576, 42543733347512427027818187985506009087, 237, 2), (42543733347512427027818187985506009088, 42544057866066085454544971141526585343, 237, 2), (42544057866066085454544971141526585344, 42544382384619743881271754297547161599, 237, 2), (42545680458834377588178886921629466624, 42550872755692912415807417417958686719, 237, 7), 
    (42550872755692912415807417417958686720, 42552170829907546122714550042040991743, 237, 10), (47852207848256971424537054170092404736, 47935284597993528666593542111359926271, 237, 2), (50510663839826803170344668290653093888, 50593740589563360412401156231920615423, 237, 3), (50593740589563360412401156231920615424, 50593781154382567715742004126423187455, 237, 3), 
    (50676817339299917654457644173188136960, 50676857904119124957798492067690708991, 237, 3), (53169119831396634916152282411213783040, 53252196581133192158208770352481304575, 237, 8), (55827575822966466661959896531774472192, 55910652572703023904016384473041993727, 237, 10), (58486031814536298407767510652335161344, 58569108564272855649823998593602682879, 237, 0), 
    (85070591730234615865843651857942052864, 127605887595351923798765477786913079295, 237, 7), (127605887595351923798765477786913079296, 170141183460469231731687303715884105727, 237, 7), (170141183460469231731687303715884105728, 212676479325586539664609129644855132159, 237, 7), (212676479325586539664609129644855132160, 255211775190703847597530955573826158591, 237, 7), 
    (255211775190703847597530955573826158592, 297747071055821155530452781502797185023, 237, 7), (297747071055821155530452781502797185024, 319014718988379809496913694467282698239, 237, 7), (319014718988379809496913694467282698240, 329648542954659136480144150949525454847, 237, 7), (329648542954659136480144150949525454848, 334965454937798799971759379190646833151, 237, 7), 
    (334965454937798799971759379190646833152, 337623910929368631717566993311207522303, 237, 7), (337623910929368631717566993311207522304, 338288524927261089654018896841347694591, 237, 7), (338288524927261089654018896841347694592, 338620831926207318622244848606417780735, 237, 7), (338620831926207318622244848606417780736, 338953138925153547590470800371487866879, 237, 7), 
    (338953138925153547590470800371487866880, 340282366920938463463374607431768211455, 237, 7), 
];