#[allow(unused_variables)]
#[macro_use]
#[allow(unused_attributes)]
#[allow(unused_parens)]

#[allow(dead_code)]
static TSWEIGHTS: &[[u32;2]] = &[
    [5, 75], [6, 75], [7, 75], [8, 75], [16, 75],
    [17, 75], [18, 75], [9, 95], [11, 95], [12, 95],
    [13, 95], [14, 95], [15, 95], [19, 110], [23, 110],
    [24, 110], [25, 110], [26, 110], [28, 104], [29, 104],
    [30, 104], [31, 104], [32, 104], [33, 104], [35, 101],
    [36, 101], [37, 101], [38, 101], [39, 101], [40, 101],
    [41, 101], [42, 101], [43, 101], [44, 101], [45, 101],
    [46, 101], [47, 101], [48, 101], [49, 101], [50, 101],
    [52, 110], [53, 110], [55, 110], [57, 110],
    [58, 110], [59, 110], [60, 110], [61, 110],
    [62, 110], [63, 110], [64, 110], [65, 110],
    [66, 110], [68, 95], [71, 116], [72, 116],
    [73, 116], [74, 116], [75, 116], [76, 116],
    [77, 116], [78, 116], [79, 116], [80, 116],
    [81, 116], [82, 116], [83, 116], [84, 116]
];

static SPECIALS: &[(&str, u32)] = &[
("mitvcanal", 56), ("animeultimacom", 34), ("cricket365live", 21),
("pokemonepisodeorg", 22), ("animelinkz", 20), ("sport24lt", 56),
("narutowire", 10), ("watchanimeonn", 22), ("cricvid-hitcric-", 51),
("narutochatt", 70), ("leeplarp", 27), ("stream2watch3", 56), ("ttvsports", 56),
("ver-anime", 8), ("vipstand", 21), ("eafangames", 56), ("soccerjumbo", 21),
("myfoxdfw", 67), ("kiiiikiii", 21), ("de-livechat", 5), ("rgsmotrisport", 51),
("dbzepisodeorg", 10), ("watch-dragonball", 8), ("peliculas-flv", 69),
("tvanimefreak", 54), ("tvtvanimefreak", 54)
];


pub fn get_server(group: &str) -> String {
    // un for, nada especial.
    for &x in SPECIALS {
        if group == x.0 {
            return format!("s{}.chatango.com", x.1);
        }
    }
    let group = group.replace("_", "q").replace("-", "q"); //remplazo de caracteres
    //['a','b', 'c','d','e',] -> (5) -> "abcde"
    let part1 = group.chars().take(5).collect::<String>();
    let fnv = u32::from_str_radix(&part1, 36).unwrap(); //numero de base 10 -> func(36)-> numero base36 
    //['a','b', 'c','d','e','f','g', 'h', 'i'] -> [...6] -> ['g', 'h', 'i'] -> "ghi" 
    let part2 = group.chars().skip(6).take(3).collect::<String>();
    let lnv = if !part2.is_empty() { //si no esta vacio?
        let parsed = u32::from_str_radix(&part2, 36).unwrap(); //b10 -> func(b36) -> numero 
        std::cmp::max(parsed, 1000) // si es mayor se toma, si es menor sera 1000
    } else { // si el string esta vacio le ponemos 1000
        1000
    };
    let num = (fnv % lnv) as f64 / lnv as f64; // (fnv modr lnv) en entero y divide / y obtiene un numero decimal
    let maxsum: u32 = TSWEIGHTS.iter().map(|x| x[1]).sum(); // [[17,75],[18,75]...] -> [75, 75...] luego suma.
    let mut cumfreq = 0.0; // frecuencia relativa
    let mut sn = 0; // server
    for x in TSWEIGHTS { //lo demas es igual a la libreria ch.py
        cumfreq += x[1] as f64 / maxsum as f64;
        if num <= cumfreq { 
            sn = x[0];
            break;
        }
    }

    format!("s{}.chatango.com", sn)
}

