// utilidades de chatango: servidores, uid, anon names, limpieza de mensajes

use rand::Rng;
use regex::Regex;

// salas con servidor especial fijo
static SPECIALS: &[(&str, u32)] = &[
    ("mitvcanal", 56), ("animeultimacom", 34), ("cricket365live", 21),
    ("pokemonepisodeorg", 22), ("animelinkz", 20), ("sport24lt", 56),
    ("narutowire", 10), ("watchanimeonn", 22), ("cricvid-hitcric-", 51),
    ("narutochatt", 70), ("leeplarp", 27), ("stream2watch3", 56), ("ttvsports", 56),
    ("ver-anime", 8), ("vipstand", 21), ("eafangames", 56), ("soccerjumbo", 21),
    ("myfoxdfw", 67), ("kiiiikiii", 21), ("de-livechat", 5), ("rgsmotrisport", 51),
    ("dbzepisodeorg", 10), ("watch-dragonball", 8), ("peliculas-flv", 69),
    ("tvanimefreak", 54), ("tvtvanimefreak", 54),
];

// pesos pa calcular que servidor le toca a una sala
static TSWEIGHTS: &[[u32; 2]] = &[
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
    [81, 116], [82, 116], [83, 116], [84, 116],
];

// calcula que servidor le toca a una sala
pub fn get_server(group: &str) -> String {
    for &(name, sn) in SPECIALS {
        if group == name {
            return format!("s{}.chatango.com", sn);
        }
    }

    let group = group.replace('_', "q").replace('-', "q");
    let part1: String = group.chars().take(5).collect();
    let fnv = u32::from_str_radix(&part1, 36).unwrap_or(0);

    let part2: String = group.chars().skip(6).take(3).collect();
    let lnv = if !part2.is_empty() {
        let parsed = u32::from_str_radix(&part2, 36).unwrap_or(1000);
        std::cmp::max(parsed, 1000)
    } else {
        1000
    };

    let num = (fnv % lnv) as f64 / lnv as f64;
    let maxsum: u32 = TSWEIGHTS.iter().map(|x| x[1]).sum();
    let mut cumfreq = 0.0;
    let mut sn = 0;
    for x in TSWEIGHTS {
        cumfreq += x[1] as f64 / maxsum as f64;
        if num <= cumfreq {
            sn = x[0];
            break;
        }
    }

    format!("s{}.chatango.com", sn)
}

// uid random de 16 digitos pa la auth
pub fn gen_uid() -> String {
    let mut rng = rand::thread_rng();
    let uid: u64 = rng.gen_range(10u64.pow(15)..10u64.pow(16));
    uid.to_string()
}

// saca el nombre de anon del puid y el tiempo de conexion
pub fn get_anon_name(ts_id: &str, anon_id: &str) -> String {
    let ts_id = if ts_id.len() < 4 {
        "3452".to_string()
    } else {
        let ts = ts_id.split('.').next().unwrap_or("3452");
        let ts = if ts.len() >= 4 {
            ts[ts.len() - 4..].to_string()
        } else {
            format!("{:0>4}", ts)
        };
        ts
    };

    let anon_id = if anon_id.len() < 4 {
        format!("{:0>8}", anon_id)
    } else if anon_id.len() > 8 {
        anon_id[..8].to_string()
    } else {
        anon_id.to_string()
    };

    let number = &anon_id[4..8.min(anon_id.len())];
    let result: String = number
        .chars()
        .zip(ts_id.chars())
        .map(|(a, b)| {
            let da = a.to_digit(10).unwrap_or(0);
            let db = b.to_digit(10).unwrap_or(0);
            char::from_digit((da + db) % 10, 10).unwrap()
        })
        .collect();

    format!("anon{}", result)
}

// quita tags html
pub fn strip_html(msg: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(msg, "").to_string()
}

// limpia un mensaje de chatango, saca el body, color del nombre, y fuente
pub fn clean_message(msg: &str) -> (String, String, String) {
    let n_re = Regex::new(r"<n(.*?)/>").unwrap();
    let f_re = Regex::new(r"<f(.*?)>").unwrap();

    let name_color = n_re
        .captures(msg)
        .map(|c| c[1].to_string())
        .unwrap_or_default();

    let font_info = f_re
        .captures(msg)
        .map(|c| c[1].to_string())
        .unwrap_or_default();

    let cleaned = n_re.replace_all(msg, "");
    let cleaned = f_re.replace_all(&cleaned, "");
    let cleaned = strip_html(&cleaned);
    let cleaned = html_escape::decode_html_entities(&cleaned)
        .replace('\r', "\n");

    (cleaned.to_string(), name_color, font_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_server() {
        assert_eq!(get_server("mitvcanal"), "s56.chatango.com");
        assert_eq!(get_server("sudoers"), get_server("sudoers"));
    }

    #[test]
    fn test_gen_uid() {
        let uid = gen_uid();
        assert_eq!(uid.len(), 16);
    }

    #[test]
    fn test_get_anon_name() {
        let name = get_anon_name("1757533649.9", "75730148");
        assert_eq!(name, "anon3787");
    }
}
