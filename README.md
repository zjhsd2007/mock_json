## Mock JSON
A very simple and easy-to-use JSON generation tool that supports customizable formats and can be extended with custom `Placeholders`.

### Example
``` rust
use mock_json::mock;
use serde_json::json;

let val = mock(&json!({
    "code":0,
    "msg":"just text",
    "data":[{
        "id":"@Id|10",
        "title": "@Title",
        "datetime":"@DateTime",
        "author":{
            "name":"@Name",
            "id":"@Guid",
            "email":"@Email",
            "id_number":"@IdNumber",
            "ip":"@Ip",
            "phones":["@Phone", 1, 3],
            "blog":"@Url",
            "avatar":"@Image|80X80|f7f7f7|fff"
        }
    }, 5, 20]
}));
```

The above code will return a `serde_json::Value`, and after calling `val.to_string()`, its content will be as follows:

```json
{
  "code": 0,
  "msg": "just text",
  "data": [
    {
      "author": {
        "avatar": "https://dummyimage.com/80X80/f7f7f7/fff",
        "blog": "https://fvopl.ktuh.int/dcvr",
        "email": "vpar@hslsl.org",
        "id": "ceE68058-5EaB-4bc2-cCc8-F4a2Dff1Fe6A",
        "id_number": "646734191701136174",
        "ip": "132.86.194.66",
        "name": "Patricia Garcia",
        "phones": [
          "13318945147",
          "14923999763"
        ]
      },
      "datetime": "1960-02-12 03:49:48",
      "id": "3217A3bAAa",
      "title": "bymebox wpvvpv udp pcb lky onigkew sywtnhq"
    },
    ...
  ]
}
```

### Generating a List of Data
To generate a list of data, you can use the format `[serde_json::Value, min, max]`, which generates a number of data greater than `min` and less than `max`.
```rust
 let val = mock(&json!([{"user_name": "@Name", "email": "@Email", "age":"@Number|18~100"}, 2, 5]));
```
then calling `val.to_string()`, its content will be:
```json
[
    {
        "age": 43,
        "email": "dbjfm@drgmz.com",
        "user_name":"Laura White"
    },
    {
        "age": 35,
        "email": "fbac@yefq.edu",
        "user_name":"Frank Hernandez"
    },
    {
        "age": 31,
        "email": "kkhbo@vbqv.cn",
        "user_name":"Jose Wilson"
    }
]
```

### Implementation
The principle is to replace the `placeholders` with the corresponding content. Each `placeholder` starts with `@`, followed by a `placeholder name`, and optional parameters separated by `|`. For example, `@Id` will generate a string of length 12 containing `[a-zA-Z0-9]`. If you want to generate an id of length 5, just use `@Id|5`.
The return value of each `placeholder` is `serde_json::Value`.


### Placeholders
The default `placeholders` provided are as follows:

#### @Guid
Randomly generates a string in the format of xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx, without any parameters.

```rust
mock(&json!("@Guid")); // String("753c33fE-fFdB-12aF-bC44-A9DD2D7E10aA")
```

#### @Id
Randomly generates a string of length 12 containing `[a-zA-Z0-9]`. If you want to generate an id of a custom length, use `@Id|5` as an example.

```rust
// Default length: 12
mock($json!("@Id")); // String("YNyxSBVzgpQd")

// Custom length
mock(&json!("@Id|5")); // String("vygCt")
```

#### @IdNumber
Randomly generates a Chinese ID number, no accept params.

```rust
mock(&json!("@IdNumber")); // String("665471198804049116")
```

#### @Phone
Randomly generates a Chinese phone number, no accept params.

```rust
mock(&json!("@Phone")); // String("13983370699")
mock(&json!("@Phone|86")); // String("+86 13545607699")
```

#### @Bool
Randomly generates a boolean value, no accept params.

```rust
mock(&json!("@Bool")); // Bool(true)
```

#### @Name
Randomly generates an English name. It can accept parameters. If you want to generate a Chinese name, you can pass the parameter `cn`.

```rust
mock(&json!("@Name")); // String("Donald Davis")

// Generate a Chinese name
mock(&jsonQ("@Name|cn")); // String("蔡茗泽")
```

#### @FirstName
Randomly generates a `first name`. It can accept parameters. If you want to generate a Chinese first name, you can pass the parameter `cn`.

```rust
mock(&json!("@FirstName")); // String("Mark")

// Generate a Chinese surname
mock(&json!("@FirstName|cn")); // String("张")
```

#### @LastName
Randomly generates a `last name`. It can accept parameters. If you want to generate a Chinese last name, you can pass the parameter `cn`.

```rust
mock(&json!("@LastName")); // String("Martin")

// Generate a Chinese last name
mock(&json!("@LastName|cn")); // String("沐宸")
```

#### @Url
Randomly generates a URL. The protocol of the URL can be customized.

```rust
mock(&json!("@Url")); // String("https://mqezx.rpmy.int/gevc")

// Customize the URL protocol
mock(&json!("@Url|ftp")); // String("ftp://qjwb.wpukq.gov/tmkwq")
```
#### @Token
Randomly generates a token

```rust
mock(&json!("@Token")); // String("htK5pesIqPTJYK8Yn286.RG0mgC0vHPlQU7SnxWx3.fHsemPcj43bEY0hJSSfJ")
```

#### @Email
Randomly generates an email address, no accept params.

```rust
mock(&json!("@Email")); // String("hhci@engu.cn")
```

#### @Ip
Randomly generates an IP address. By default, the IP format is IPv4, but it also supports generating IPv6 addresses by passing in `ipV6`.

```rust
// ipv4
mock(&json!("@Ip")); // String("161.26.255.122")

// ipv6
mock(&json!("@Ip|ipV6")); // String("D991:53F9:2131:D6CA:86DA:56C4:156D:91B0")
```

#### @Domain
Randomly generates a domain name, no accept params.

```rust
mock(&json!("@Domain")); // String("hxca.gov")
```
#### @Paragraph
Randomly generates a paragraph. To generate a Chinese paragraph, pass in `cn`.

```rust
mock(&json!("@Paragraph")); // String("eetcttp jekaveq uwzkl abzciz bquijz biq ajfbnsjx ljmy khx,..., iilxxy mudlb xjz eipnm wnuc, takktis dus vwmubs ysckswn bju uffje.")

// Chinese paragraph
mock(&json!("@Paragraph|cn")); // String("却国华资称被素火用几花放西等，...，层间者场当族表眼日技里度，际感高一声新历院器火常问书则如。")
```

#### @Sentence
Randomly generates a sentence, To generate a Chinese sentence, pass in `cn`
```rust
mock(&json!("@Sentence")); //String("zcdh yiui qymurgx szaydjv yfb tkj znpeyy muzwrs okihyo")

// Chinese sentence
mock(&json!("@Sentence|cn")) // String("型有件次一通说存克这半确毛由")
```

#### @Title
Randomly generates a title. To generate a Chinese title, pass in `cn`. It is similar to `@Sentence`.

```rust
mock(&json!("@Title")); // String("fivms iiqq kdvyojq mibvzkx efhi six zpogksf")

// Chinese title
mock(&json!("@Title|cn")); // String("别话省报回京老住基")
```

#### @Image
Generates a URL for an image provided by [https://dummyimage.com](https://dummyimage.com). The size, background color, and foreground color can be specified, but it is not possible to define only the background or foreground color.

```rust
// Default image size is 320X240, with black background and white foreground
mock(&json!("@Image"));  // String("https://dummyimage.com/320X240/000/fff")

// Custom image size
mock(&json!("@Image|40X40")); // String("https://dummyimage.com/40X40/000/fff")

// Custom size and background color
mock(&json!("@Image|40X40|f00")); // String("https://dummyimage.com/40X40/f00/fff")

// Custom size, background color, and foreground color
mock(&json!("@Image|40X40|f00|fea")); // String("https://dummyimage.com/40X40/f00/fea")
```

#### @Number
Randomly generates a natural number between 0 and 1000 by default. Accepts parameters to specify a range using `@Number|min~max`.

```rust
mock(&json!("@Number")); // Number(326)

mock(&json!("@Number|100~999")); //Number(437)

mock(&json!("@Number|-200~-100")); // Number(-147)
```

#### @Float
Randomly generates a floating-point number. Accepts two optional parameters. When there is only one parameter, if the parameter contains `~`, it represents the range of values, otherwise, it represents the exact number of decimal places. When there are two parameters, the first parameter represents the exact number of decimal places, and the second parameter represents the range of values.


```rust
// default precision is 2
mock(&json!("@Float")); // Number(0.13)

// Specify precision
mock(&json!("@Float|3")); // Number(0.628)

// Specify value range
mock(&json!("@Float|100~999")); // Number(123.56)

// Specify precision and value range, it is not possible to define only the value range.
mock(&json!("@Float|2|100~999")); // Number(131.99)
```

#### @Zip
Randomly generates a postal code.

```rust
mock(&json!("@Zip")); // String("859615")
```

#### @Address
Randomly generates an address. Pass in `cn` to generate a Chinese address.

```rust
mock(&json!("@Address")); //String("2454 Kidd Avenue Anchorage Alaska")

// Generate a Chinese address
mock(&json!("@Address|cn")); //String("湖北省武汉市汉阳区仙山社区")
```

#### @Date
Randomly generates a date. The format can be customized.

```rust
mock(&json!("@Date")); //String("1984-09-28")

// Custom date format
mock(&json!("@Date|YYYY/MM/DD")); //String("1918/01/13")

mock(&json!("@Date|YY/M/D")); //String("36/2/1")
```

#### @Time
Randomly generates a time.

```rust
mock(&json!("@Time")); //String("09:58:54")
```

#### @DateTime
Randomly generates a date and time. The format can be customized.

```rust
mock(&json!("@DateTime")); //String("1937-06-24 10:47:53")

// Custom format
mock(&json!("@DateTime|YYYY/MM/DD hh:mm:ss")); //String("2007/01/23 16:52:03")
```

#### @Timestamp
Randomly generates a timestamp.

```rust
mock(&json!("@Timestamp")); //Number(1812982294012)
```

#### @Color
Randomly generates a color in hex format.

```rust
mock(&json!("@Color")); //String("#F4F54B")
```

#### @RGB
Randomly generates a color in RGB format.

```rust
mock(&json!("@RGB")); //String("rgb(79,134,9)")
```

#### @RGBA
Randomly generates a color in RGBA format.

```rust
mock(&json!("@RGBA")); //String("rgba(240,131,198,0.65)")
```

#### @HSL
Randomly generates a color in HSL format.

```rust
mock(&json!("@HSL")); //String("hsl(343,50,32)")
```

<br/>

#### Register your `placeholders`
The above are all the `placeholders` currently available. You can also register your own `placeholders` through the `registry` function, it must implements the `MockFn` trait. However, you cannot register `placeholders` that already exist. Although there is no strict naming convention for `placeholders`, it is still recommended to use camel case. Except for abbreviations like `RGB`. <br/><br/>
For example: Now I want to register a `placeholder` called `@CMYK`, which is used to generate color in cmyk format.

```rust
pub struct MockCMYKFn;
impl MockFn for MockCMYKFn {
    // Ignore the parameter since it is not used here.
    fn mock(&self, _:Option<Vec<&str>>) -> Value {
        let mut rng = rand::thread_rng();
        let c = rng.gen_range(0..=100);
        let m = rng.gen_range(0..=100);
        let y = rng.gen_range(0..=100);
        let k = rng.gen_range(0..=100);
        json!(format!("cmyk({},{},{},{})", c, m, y, k))
    }
}

// Register
registry("@CMYK", MockCMYKFn);

// Usage
mock(&json!("@CMYK")); // String("cmyk(99,20,87,54)))
```
