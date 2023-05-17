use once_cell::sync::Lazy;

pub const STR_POOL: Lazy<Vec<char>> = Lazy::new(|| "abcdefABCDEF0123456789".chars().collect());
pub const ALL_LETTER_POOL: Lazy<Vec<char>> = Lazy::new(|| {
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect()
});
pub const NUMBER_POOL: Lazy<Vec<char>> = Lazy::new(|| "0123456789".chars().collect());
pub const LETTER_POOL: Lazy<Vec<char>> =
    Lazy::new(|| "abcdefghijklmnopqrstuvwxyz".chars().collect());
pub const CN_LETTER_POOL: Lazy<Vec<char>> = Lazy::new(|| {
    "的一是在不了有和人这中大为上个国我以要他时来用们生到作地于出就分对成会可主发年动同工也能下过子说产种面而方后多定行学法所民得经十三之进着等部度家电力里如水化高自二理起小物现实加量都两体制机当使点从业本去把性好应开它合还因由其些然前外天政四日那社义事平形相全表间样与关各重新线内数正心反你明看原又么利比或但质气第向道命此变条只没结解问意建月公无系军很情者最立代想已通并提直题党程展五果料象员革位入常文总次品式活设及管特件长求老头基资边流路级少图山统接知较将组见计别她手角期根论运农指几九区强放决西被干做必战先回则任取据处队南给色光门即保治北造百规热领七海口东导器压志世金增争济阶油思术极交受联什认六共权收证改清己美再采转更单风切打白教速花带安场身车例真务具万每目至达走积示议声报斗完类八离华名确才科张信马节话米整空元况今集温传土许步群广石记需段研界拉林律叫且究观越织装影算低持音众书布复容儿须际商非验连断深难近矿千周委素技备半办青省列习响约支般史感劳便团往酸历市克何除消构府称太准精值号率族维划选标写存候毛亲快效斯院查江型眼王按格养易置派层片始却专状育厂京识适属圆包火住调满县局照参红细引听该铁价严龙飞".chars().collect()
});
pub const PHONE_PREFIX: [&str; 6] = ["133", "149", "153", "159", "135", "139"];
pub const FIRST_NAME: [&str; 66] = [
    "James",
    "John",
    "Robert",
    "Michael",
    "William",
    "David",
    "Richard",
    "Charles",
    "Joseph",
    "Thomas",
    "Christopher",
    "Daniel",
    "Paul",
    "Mark",
    "Donald",
    "George",
    "Kenneth",
    "Steven",
    "Edward",
    "Brian",
    "Ronald",
    "Anthony",
    "Kevin",
    "Jason",
    "Matthew",
    "Gary",
    "Timothy",
    "Jose",
    "Larry",
    "Jeffrey",
    "Frank",
    "Scott",
    "Eric",
    "Mary",
    "Patricia",
    "Linda",
    "Barbara",
    "Elizabeth",
    "Jennifer",
    "Maria",
    "Susan",
    "Margaret",
    "Dorothy",
    "Lisa",
    "Nancy",
    "Karen",
    "Betty",
    "Helen",
    "Sandra",
    "Donna",
    "Carol",
    "Ruth",
    "Sharon",
    "Michelle",
    "Laura",
    "Sarah",
    "Kimberly",
    "Deborah",
    "Jessica",
    "Shirley",
    "Cynthia",
    "Angela",
    "Melissa",
    "Brenda",
    "Amy",
    "Anna",
];
pub const LAST_NAME: [&str; 32] = [
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Miller",
    "Davis",
    "Garcia",
    "Rodriguez",
    "Wilson",
    "Martinez",
    "Anderson",
    "Taylor",
    "Thomas",
    "Hernandez",
    "Moore",
    "Martin",
    "Jackson",
    "Thompson",
    "White",
    "Lopez",
    "Lee",
    "Gonzalez",
    "Harris",
    "Clark",
    "Lewis",
    "Robinson",
    "Walker",
    "Perez",
    "Hall",
    "Young",
    "Allen",
];

pub const CN_FIRST_NAME: [&str; 44] = [
    "陈", "林", "周", "张", "李", "黄", "王", "吴", "刘", "蔡", "杨", "许", "郑", "谢", "郭", "洪",
    "邱", "曾", "廖", "赖", "徐", "周", "叶", "苏", "庄", "江", "吕", "何", "罗", "高", "萧", "潘",
    "朱", "简", "锺", "彭", "游", "詹", "胡", "施", "沉", "余", "赵", "卢",
];

pub const CN_LAST_NAME: [&str; 20] = [
    "沐宸", "浩宇", "沐辰", "茗泽", "奕辰", "宇泽", "浩然", "奕泽", "宇轩", "沐阳", "若汐", "一诺",
    "艺涵", "依诺", "梓涵", "苡沫", "雨桐", "欣怡", "语桐", "语汐",
];

pub const WEB: [&str; 8] = ["com", "net", "org", "edu", "gov", "int", "mil", "cn"];
