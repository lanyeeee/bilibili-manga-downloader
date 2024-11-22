use base64::engine::general_purpose;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn filename_filter(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' | '/' => ' ',
            ':' => '：',
            '*' => '⭐',
            '?' => '？',
            '"' => '\'',
            '<' => '《',
            '>' => '》',
            '|' => '丨',
            '.' => '·',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn generate_android_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 8] = rng.gen(); // 生成 8 字节（64 位）的随机数
    hex::encode(random_bytes) // 将随机字节转换为十六进制字符串
}

pub fn gen_aurora_eid(uid: u64) -> String {
    if uid == 0 {
        return String::new();
    }
    let mut result_byte = Vec::with_capacity(64);
    // 1. 将 UID 字符串转为字节数组.
    let mid_byte = uid.to_string().into_bytes();
    // 2. 将字节数组逐位(记为第 i 位)与 b"ad1va46a7lza" 中第 (i % 12) 位进行异或操作, 作为结果数组第 i 位.
    mid_byte.iter().enumerate().for_each(|(i, v)| {
        result_byte.push(v ^ (b"ad1va46a7lza"[i % 12]));
    });
    // 3. 对字节数组执行 Base64 编码, 注意 no padding, 即得到 x-bili-aurora-eid.
    general_purpose::STANDARD.encode(result_byte)
}

#[allow(clippy::cast_possible_truncation)]
pub fn gen_trace_id() -> String {
    // 1. 生成 32 位随机字符串 random_id , Charset 为 0~9, a~z.
    let random_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(u8::is_ascii_alphanumeric) // 过滤掉非字母数字字符
        .take(32)
        .map(char::from)
        .collect();
    let mut random_trace_id = String::with_capacity(40);
    // 2. 取 random_id 前 24 位, 作为 random_trace_id.
    random_trace_id.push_str(&random_id[0..24]);
    // 3. 初始化一个长度为 3 的数组 b_arr, 初始值都为 0.
    let mut b_arr: [i8; 3] = [0i8; 3];
    // 并获取当前时间戳
    let mut ts = chrono::Local::now().timestamp();
    // 使用循环从高位到低位遍历 b_arr 数组, 循环体内执行以下逻辑:
    //  - 首先将 ts 右移 8 位
    //  - 然后根据条件向 b_arr 的第 i 位赋值:
    //    - 如果 (ts / 128) % 2的结果为0, 则 b_arr[i] = ts % 256
    //    - 否则 b_arr[i] = ts % 256 - 256
    for i in (0..3).rev() {
        ts >>= 8;
        b_arr[i] = {
            if ((ts / 128) % 2) == 0 {
                (ts % 256) as i8
            } else {
                (ts % 256 - 256) as i8
            }
        }
    }
    // 4. 将数组 b_arr 中的每个元素逐个转换为两位的十六进制字符串并追加到 random_trace_id 中.
    for i in 0..3 {
        random_trace_id.push_str(&format!("{:0>2x}", b_arr[i]));
    }
    // 5. 将 random_id 的第 31, 32 个字符追加到 random_trace_id 中, 此时 random_trace_id 生成完毕, 应当为 32 位长度.
    random_trace_id.push_str(&random_id[30..32]);
    // 6. 最后, 按 `{random_trace_id}:{random_trace_id[16..32]}:0:0` 的顺序拼接起来, 即为 x-bili-trace-id
    let mut random_trace_id_final = String::with_capacity(64);
    random_trace_id_final.push_str(&random_trace_id);
    random_trace_id_final.push(':');
    random_trace_id_final.push_str(&random_trace_id[16..32]);
    random_trace_id_final.push_str(":0:0");
    random_trace_id_final
}

pub fn gen_session_id() -> String {
    // 保证全都是小写
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(u8::is_ascii_alphanumeric) // 过滤掉非字母数字字符
        .take(8)
        .map(|c| c.to_ascii_lowercase())
        .map(char::from)
        .collect()
}
