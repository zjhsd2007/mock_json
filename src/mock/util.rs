use rand::Rng;
pub fn create_ip_v4() -> String {
    format!(
        "{}.{}.{}.{}",
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    )
}
pub fn create_ip_v6() -> String {
    let ip_v6_field = || format!("{:02X}{:02X}", rand::random::<u8>(), rand::random::<u8>());
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field(),
        ip_v6_field()
    )
}

pub fn pick(origin: &[&'static str]) -> String {
    let len = origin.len() as u32;
    origin
        .get((rand::random::<u32>() % len) as usize)
        .map(|v| v.to_string())
        .unwrap()
}

pub fn pick_str(origin: &Vec<char>, len: u32) -> String {
    let char_len = origin.len();
    let mut re = vec![];
    for _ in 0..len {
        let idx = rand::random::<u32>() % char_len as u32;
        if let Some(&c) = origin.get(idx as usize) {
            re.push(c)
        }
    }
    re.iter().collect::<String>()
}

pub fn create_number(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn create_word(origin: &Vec<char>, min: u8, max: u8) -> String {
    let mut re = vec![];
    let len = origin.len() as u32;
    let mut rng = rand::thread_rng();
    let count = rng.gen_range(min..max);
    for _ in 0..=count {
        re.push(*(origin.get((rand::random::<u32>() % len) as usize).unwrap()));
    }
    re.iter().collect::<String>()
}

pub fn create_sentence(origin: &Vec<char>, min: u8, max: u8) -> String {
    let mut re = vec![];
    let mut rng = rand::thread_rng();
    let count = rng.gen_range(min..max);
    for _ in 0..=count {
        re.push(create_word(origin, 2, 7));
    }
    re.join(" ")
}

pub fn format_datetime(
    format: &str,
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> String {
    format
        .replace("YYYY", year.to_string().as_str())
        .replace("YY", &year.to_string()[2..])
        .replace("MM", format!("{:02}", month).as_str())
        .replace("M", month.to_string().as_str())
        .replace("DD", format!("{:02}", day).as_str())
        .replace("D", day.to_string().as_str())
        .replace("hh", format!("{:02}", hour).as_str())
        .replace("h", hour.to_string().as_str())
        .replace("mm", format!("{:02}", minute).as_str())
        .replace("m", minute.to_string().as_str())
        .replace("ss", format!("{:02}", second).as_str())
        .replace("s", second.to_string().as_str())
}
