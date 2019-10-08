// -*- mode: rust; -*-
//
// This file is part of `fancy-garbling`.
// Copyright © 2019 Galois, Inc.
// See LICENSE for licensing information.

use crate::util;

pub fn lookup(q: u16) -> Vec<u128> {
    match q {
        5 => vec![
            1,
            5,
            25,
            125,
            625,
            3125,
            15625,
            78125,
            390625,
            1953125,
            9765625,
            48828125,
            244140625,
            1220703125,
            6103515625,
            30517578125,
            152587890625,
            762939453125,
            3814697265625,
            19073486328125,
            95367431640625,
            476837158203125,
            2384185791015625,
            11920928955078125,
            59604644775390625,
            298023223876953125,
            1490116119384765625,
            7450580596923828125,
            37252902984619140625,
            186264514923095703125,
            931322574615478515625,
            4656612873077392578125,
            23283064365386962890625,
            116415321826934814453125,
            582076609134674072265625,
            2910383045673370361328125,
            14551915228366851806640625,
            72759576141834259033203125,
            363797880709171295166015625,
            1818989403545856475830078125,
            9094947017729282379150390625,
            45474735088646411895751953125,
        ],
        6 => vec![
            1,
            6,
            36,
            216,
            1296,
            7776,
            46656,
            279936,
            1679616,
            10077696,
            60466176,
            362797056,
            2176782336,
            13060694016,
            78364164096,
            470184984576,
            2821109907456,
            16926659444736,
            101559956668416,
            609359740010496,
            3656158440062976,
            21936950640377856,
            131621703842267136,
            789730223053602816,
            4738381338321616896,
            28430288029929701376,
            170581728179578208256,
            1023490369077469249536,
            6140942214464815497216,
            36845653286788892983296,
            221073919720733357899776,
            1326443518324400147398656,
            7958661109946400884391936,
            47751966659678405306351616,
            286511799958070431838109696,
            1719070799748422591028658176,
            10314424798490535546171949056,
            61886548790943213277031694336,
            371319292745659279662190166016,
            2227915756473955677973140996096,
            13367494538843734067838845976576,
            80204967233062404407033075859456,
        ],
        7 => vec![
            1,
            7,
            49,
            343,
            2401,
            16807,
            117649,
            823543,
            5764801,
            40353607,
            282475249,
            1977326743,
            13841287201,
            96889010407,
            678223072849,
            4747561509943,
            33232930569601,
            232630513987207,
            1628413597910449,
            11398895185373143,
            79792266297612001,
            558545864083284007,
            3909821048582988049,
            27368747340080916343,
            191581231380566414401,
            1341068619663964900807,
            9387480337647754305649,
            65712362363534280139543,
            459986536544739960976801,
            3219905755813179726837607,
            22539340290692258087863249,
            157775382034845806615042743,
            1104427674243920646305299201,
            7730993719707444524137094407,
            54116956037952111668959660849,
            378818692265664781682717625943,
            2651730845859653471779023381601,
            18562115921017574302453163671207,
            129934811447123020117172145698449,
            909543680129861140820205019889143,
            6366805760909027985741435139224001,
            44567640326363195900190045974568007,
        ],
        9 => vec![
            1,
            9,
            81,
            729,
            6561,
            59049,
            531441,
            4782969,
            43046721,
            387420489,
            3486784401,
            31381059609,
            282429536481,
            2541865828329,
            22876792454961,
            205891132094649,
            1853020188851841,
            16677181699666569,
            150094635296999121,
            1350851717672992089,
            12157665459056928801,
            109418989131512359209,
            984770902183611232881,
            8862938119652501095929,
            79766443076872509863361,
            717897987691852588770249,
            6461081889226673298932241,
            58149737003040059690390169,
            523347633027360537213511521,
            4710128697246244834921603689,
            42391158275216203514294433201,
            381520424476945831628649898809,
        ],
        10 => vec![
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000,
            100000000000,
            1000000000000,
            10000000000000,
            100000000000000,
            1000000000000000,
            10000000000000000,
            100000000000000000,
            1000000000000000000,
            10000000000000000000,
            100000000000000000000,
            1000000000000000000000,
            10000000000000000000000,
            100000000000000000000000,
            1000000000000000000000000,
            10000000000000000000000000,
            100000000000000000000000000,
            1000000000000000000000000000,
            10000000000000000000000000000,
            100000000000000000000000000000,
            1000000000000000000000000000000,
            10000000000000000000000000000000,
        ],
        11 => vec![
            1,
            11,
            121,
            1331,
            14641,
            161051,
            1771561,
            19487171,
            214358881,
            2357947691,
            25937424601,
            285311670611,
            3138428376721,
            34522712143931,
            379749833583241,
            4177248169415651,
            45949729863572161,
            505447028499293771,
            5559917313492231481,
            61159090448414546291,
            672749994932560009201,
            7400249944258160101211,
            81402749386839761113321,
            895430243255237372246531,
            9849732675807611094711841,
            108347059433883722041830251,
            1191817653772720942460132761,
            13109994191499930367061460371,
            144209936106499234037676064081,
            1586309297171491574414436704891,
            17449402268886407318558803753801,
            191943424957750480504146841291811,
        ],
        12 => vec![
            1,
            12,
            144,
            1728,
            20736,
            248832,
            2985984,
            35831808,
            429981696,
            5159780352,
            61917364224,
            743008370688,
            8916100448256,
            106993205379072,
            1283918464548864,
            15407021574586368,
            184884258895036416,
            2218611106740436992,
            26623333280885243904,
            319479999370622926848,
            3833759992447475122176,
            46005119909369701466112,
            552061438912436417593344,
            6624737266949237011120128,
            79496847203390844133441536,
            953962166440690129601298432,
            11447545997288281555215581184,
            137370551967459378662586974208,
            1648446623609512543951043690496,
            19781359483314150527412524285952,
            237376313799769806328950291431424,
            2848515765597237675947403497177088,
        ],
        13 => vec![
            1,
            13,
            169,
            2197,
            28561,
            371293,
            4826809,
            62748517,
            815730721,
            10604499373,
            137858491849,
            1792160394037,
            23298085122481,
            302875106592253,
            3937376385699289,
            51185893014090757,
            665416609183179841,
            8650415919381337933,
            112455406951957393129,
            1461920290375446110677,
            19004963774880799438801,
            247064529073450392704413,
            3211838877954855105157369,
            41753905413413116367045797,
            542800770374370512771595361,
            7056410014866816666030739693,
            91733330193268616658399616009,
            1192533292512492016559195008117,
            15502932802662396215269535105521,
            201538126434611150798503956371773,
            2619995643649944960380551432833049,
            34059943367449284484947168626829637,
        ],
        14 => vec![
            1,
            14,
            196,
            2744,
            38416,
            537824,
            7529536,
            105413504,
            1475789056,
            20661046784,
            289254654976,
            4049565169664,
            56693912375296,
            793714773254144,
            11112006825558016,
            155568095557812224,
            2177953337809371136,
            30491346729331195904,
            426878854210636742656,
            5976303958948914397184,
            83668255425284801560576,
            1171355575953987221848064,
            16398978063355821105872896,
            229585692886981495482220544,
            3214199700417740936751087616,
            44998795805848373114515226624,
            629983141281877223603213172736,
            8819763977946281130444984418304,
            123476695691247935826229781856256,
            1728673739677471101567216945987584,
            24201432355484595421941037243826176,
            338820052976784335907174521413566464,
        ],
        15 => vec![
            1,
            15,
            225,
            3375,
            50625,
            759375,
            11390625,
            170859375,
            2562890625,
            38443359375,
            576650390625,
            8649755859375,
            129746337890625,
            1946195068359375,
            29192926025390625,
            437893890380859375,
            6568408355712890625,
            98526125335693359375,
            1477891880035400390625,
            22168378200531005859375,
            332525673007965087890625,
            4987885095119476318359375,
            74818276426792144775390625,
            1122274146401882171630859375,
            16834112196028232574462890625,
            252511682940423488616943359375,
            3787675244106352329254150390625,
            56815128661595284938812255859375,
            852226929923929274082183837890625,
            12783403948858939111232757568359375,
            191751059232884086668491363525390625,
            2876265888493261300027370452880859375,
        ],
        17 => vec![
            1,
            17,
            289,
            4913,
            83521,
            1419857,
            24137569,
            410338673,
            6975757441,
            118587876497,
            2015993900449,
            34271896307633,
            582622237229761,
            9904578032905937,
            168377826559400929,
            2862423051509815793,
            48661191875666868481,
            827240261886336764177,
            14063084452067724991009,
            239072435685151324847153,
            4064231406647572522401601,
            69091933913008732880827217,
            1174562876521148458974062689,
            19967568900859523802559065713,
            339448671314611904643504117121,
        ],
        18 => vec![
            1,
            18,
            324,
            5832,
            104976,
            1889568,
            34012224,
            612220032,
            11019960576,
            198359290368,
            3570467226624,
            64268410079232,
            1156831381426176,
            20822964865671168,
            374813367582081024,
            6746640616477458432,
            121439531096594251776,
            2185911559738696531968,
            39346408075296537575424,
            708235345355337676357632,
            12748236216396078174437376,
            229468251895129407139872768,
            4130428534112329328517709824,
            74347713614021927913318776832,
            1338258845052394702439737982976,
        ],
        19 => vec![
            1,
            19,
            361,
            6859,
            130321,
            2476099,
            47045881,
            893871739,
            16983563041,
            322687697779,
            6131066257801,
            116490258898219,
            2213314919066161,
            42052983462257059,
            799006685782884121,
            15181127029874798299,
            288441413567621167681,
            5480386857784802185939,
            104127350297911241532841,
            1978419655660313589123979,
            37589973457545958193355601,
            714209495693373205673756419,
            13569980418174090907801371961,
            257829627945307727248226067259,
            4898762930960846817716295277921,
        ],
        20 => vec![
            1,
            20,
            400,
            8000,
            160000,
            3200000,
            64000000,
            1280000000,
            25600000000,
            512000000000,
            10240000000000,
            204800000000000,
            4096000000000000,
            81920000000000000,
            1638400000000000000,
            32768000000000000000,
            655360000000000000000,
            13107200000000000000000,
            262144000000000000000000,
            5242880000000000000000000,
            104857600000000000000000000,
            2097152000000000000000000000,
            41943040000000000000000000000,
            838860800000000000000000000000,
            16777216000000000000000000000000,
        ],
        21 => vec![
            1,
            21,
            441,
            9261,
            194481,
            4084101,
            85766121,
            1801088541,
            37822859361,
            794280046581,
            16679880978201,
            350277500542221,
            7355827511386641,
            154472377739119461,
            3243919932521508681,
            68122318582951682301,
            1430568690241985328321,
            30041942495081691894741,
            630880792396715529789561,
            13248496640331026125580781,
            278218429446951548637196401,
            5842587018385982521381124421,
            122694327386105632949003612841,
            2576580875108218291929075869661,
            54108198377272584130510593262881,
        ],
        22 => vec![
            1,
            22,
            484,
            10648,
            234256,
            5153632,
            113379904,
            2494357888,
            54875873536,
            1207269217792,
            26559922791424,
            584318301411328,
            12855002631049216,
            282810057883082752,
            6221821273427820544,
            136880068015412051968,
            3011361496339065143296,
            66249952919459433152512,
            1457498964228107529355264,
            32064977213018365645815808,
            705429498686404044207947776,
            15519448971100888972574851072,
            341427877364219557396646723584,
            7511413302012830262726227918848,
            165251092644282265779977014214656,
        ],
        23 => vec![
            1,
            23,
            529,
            12167,
            279841,
            6436343,
            148035889,
            3404825447,
            78310985281,
            1801152661463,
            41426511213649,
            952809757913927,
            21914624432020321,
            504036361936467383,
            11592836324538749809,
            266635235464391245607,
            6132610415680998648961,
            141050039560662968926103,
            3244150909895248285300369,
            74615470927590710561908487,
            1716155831334586342923895201,
            39471584120695485887249589623,
            907846434775996175406740561329,
            20880467999847912034355032910567,
            480250763996501976790165756943041,
        ],
        24 => vec![
            1,
            24,
            576,
            13824,
            331776,
            7962624,
            191102976,
            4586471424,
            110075314176,
            2641807540224,
            63403380965376,
            1521681143169024,
            36520347436056576,
            876488338465357824,
            21035720123168587776,
            504857282956046106624,
            12116574790945106558976,
            290797794982682557415424,
            6979147079584381377970176,
            167499529910025153071284224,
            4019988717840603673710821376,
            96479729228174488169059713024,
            2315513501476187716057433112576,
            55572324035428505185378394701824,
            1333735776850284124449081472843776,
        ],
        // TODO: add more
        _ => {
            let ndigits = util::digits_per_u128(q);
            let mut npaths_tab = vec![1; ndigits];
            for i in 1..ndigits {
                npaths_tab[i] = npaths_tab[i - 1] * q as u128;
            }
            npaths_tab
        }
    }
}
