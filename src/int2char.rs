use std::{env, process::exit};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1
    {
        println!("Usage: int2char [INTEGER]...\n\
				\tconvert INTEGER to UTF-8 - negatives (< 0) and larges (>= 2,097,152) will be ignored.\n");
        exit(0);
    }


    for element in &args[1..] {
        match string_to_natural(&element) {
            Some(t) => {
                if t < 256 {
                    print!("{}", t as u8 as char);
                } else if t < 2048 {
                    let arr = [(192 + (t >> 6)) as u8, (128 + t % 64) as u8,0,0];
                    print!("{}", String::from_utf8_lossy(&arr));
                } else if t < 65536 {
                    let arr = [(224 + (t >> 12)) as u8 , (128 + (t >> 6) % 64) as u8, (128 + t % 64) as u8,0];
                    print!("{}", String::from_utf8_lossy(&arr));
                } else if t < 2097152 {
                    let arr = [(240 + (t >> 18)) as u8 , (128 + (t >> 12) % 64) as u8, (128 + (t >> 6) % 64) as u8, (128 + t % 64) as u8];
                    print!("{}", String::from_utf8_lossy(&arr));
                } else {
                    eprintln!("Too large: {t}");
                }
                0
            },
            None => 0,
        };
    }

    exit(0);
}

fn exp10(v: u8) -> u32 {
    let mut c: u32 = 1;
    for _ in 1..v + 1 {
        c *= 10;
    }
    c
}

fn string_to_natural(str: &String) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut count:u8 = 0;
    let str = str.trim_start_matches("0");
    for ch in str.bytes().rev() {
        sum +=  exp10(count) * (match ch {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => { return None; }
        });
        count += 1;
    }
    Some(sum)
}
