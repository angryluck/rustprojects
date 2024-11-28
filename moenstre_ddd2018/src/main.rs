// #![allow(unused)]
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
// use std::io::stdin;
// use std::io::stdout;
// TODO

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // HERE ARE SOME FUNCTIONS TO USE!!!!
    //
    // string.split(' ') --> can iterate over each character, without spaces
    //
    // string.chars() --> can iterate over each character
    //
    //^^^^ These returns iterators. Can either use "for ... in ..." or do
    // something, and then apply .next()
    //
    // .unwrap() ????

    // let test_string: &str = "4\n\
    // 0 1 0 1\n\
    // 0 1 1 0\n\
    // 1 0 0 1\n\
    // 1 1 1 0\n\
    // 1 1 0 0\n\
    // 1 0 1 1\n\
    // 1 0 1 0\n\
    // 0 1 0 1";

    // let _test_string: &str = "4\n\
    // 0 1 0 1\n\
    // 0 1 1 0\n\
    // 1 0 0 1\n\
    // 1 1 1 0\n\
    // 1 0 1 0\n\
    // 0 1 1 0\n\
    // 1 0 0 1\n\
    // 0 1 1 1";

    let (size, strings) = read_input();

    let (moenster_in, moenster_out) = vec_to_pattern(size, &strings);

    let result = check_transformation(moenster_in, moenster_out);
    // let stdout = io::stdout();
    // let mut handle = stdout.lock();
    // writeln!(handle, "{}", result).expect("oh no");

    writeln!(io::stdout(), "{}", result).expect("oh no");
}

// USEFUL IN OTHER PROJECTS TOO, PROBABLY
fn read_input() -> (u32, Vec<String>) {
    let mut output_strings: Vec<String> = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let size = lines.next().expect("error").expect("errrroor");
    let size = size.parse::<u32>().expect("not a number");
    for _index in 0..2 * size {
        let last_input = lines.next().expect("arg").expect("argx2");
        output_strings.push(last_input)
    }

    return (size, output_strings);
}

fn _pretty_print(size: u32, moenster: &HashMap<(i32, i32), u8>) {
    let mut output_string = String::new();

    // FOR IF WE WANT SIZE AT TOP
    // let size_char: String = size.to_string();
    // output_string.push_str(&size_char);
    // output_string.push('\n');

    for y in (0..size as i32).rev() {
        let y = 2 * y + 1 - (size as i32);
        for x in 0..size as i32 {
            let x = 2 * x + 1 - size as i32;
            let new_index = moenster[&(x, y)].to_string();
            output_string.push_str(&new_index);
            output_string.push(' ');
        }
        output_string.push('\n')
    }
    // TRIM TRAILING '\n'
    // output_string.pop();
    println!("{}", output_string);
}

fn vec_to_pattern(
    size: u32,
    pattern_lines: &Vec<String>,
) -> (HashMap<(i32, i32), u8>, HashMap<(i32, i32), u8>) {
    // Returns size of pattern, the first pattern, and the second pattern
    let mut moenster_before = HashMap::new();
    let mut moenster_after = HashMap::new();
    let mut lines = pattern_lines.iter();

    for y in (0..size as i32).rev() {
        let y = 2 * y + 1 - size as i32;
        let mut line = lines.next().unwrap().split(' ');
        for x in 0..size as i32 {
            let x = 2 * x + 1 - size as i32;
            let tile = line.next().unwrap();
            let tile = tile.parse::<u8>().unwrap();
            moenster_before.insert((x, y), tile);
        }
    }
    for y in (0..size as i32).rev() {
        let y = 2 * y + 1 - size as i32;
        let mut line = lines.next().unwrap().split(' ');
        for x in 0..size as i32 {
            let x = 2 * x + 1 - size as i32;
            let tile = line.next().unwrap();
            let tile = tile.parse::<u8>().unwrap();
            moenster_after.insert((x, y), tile);
        }
    }

    return (moenster_before, moenster_after);
}

// STILL WANTED THIS TO WORK ;(
// fn read_pattern(size: u32, entries: &mut dyn Iterator<Item = &str>)
// -> HashMap<(i32, i32), u8> {
//     let mut moenster = HashMap::new();
//     for y in 0..size as i32 {
//         for x in 0..size as i32 {
//             // better as vectors just in the start:
//             let x = 2 * x - size as i32;
//             let y = 2 * y - size as i32;
//             let tile = (*entries).next().unwrap();
//             let tile = tile.parse::<u8>().unwrap();
//             moenster.insert((x, y), tile);
//         }
//     }
//     return moenster
// }

fn rotate_90(moenster: &HashMap<(i32, i32), u8>) -> HashMap<(i32, i32), u8> {
    let mut moenster_out = HashMap::new();
    for ((x, y), val) in moenster.iter() {
        let new_key = (*y, -*x);
        let val = *val;
        moenster_out.insert(new_key, val);
    }
    return moenster_out;
}
fn rotate_180(moenster: &HashMap<(i32, i32), u8>) -> HashMap<(i32, i32), u8> {
    return rotate_90(&rotate_90(&moenster));
}

fn rotate_270(moenster: &HashMap<(i32, i32), u8>) -> HashMap<(i32, i32), u8> {
    return rotate_90(&rotate_90(&rotate_90(&moenster)));
}

fn reflect(moenster: &HashMap<(i32, i32), u8>) -> HashMap<(i32, i32), u8> {
    let mut moenster_out = HashMap::new();
    for ((x, y), val) in moenster.iter() {
        let new_key = (-*x, *y);
        let val = *val;
        moenster_out.insert(new_key, val);
    }
    return moenster_out;
}

fn check_transformation(
    moenster_in: HashMap<(i32, i32), u8>,
    moenster_out: HashMap<(i32, i32), u8>,
) -> u8 {
    if rotate_90(&moenster_in) == moenster_out {
        return 1;
    } else if rotate_180(&moenster_in) == moenster_out {
        return 2;
    } else if rotate_270(&moenster_in) == moenster_out {
        return 3;
    } else if reflect(&moenster_in) == moenster_out {
        return 4;
    } else if rotate_90(&reflect(&moenster_in)) == moenster_out
        || rotate_180(&reflect(&moenster_in)) == moenster_out
        || rotate_270(&reflect(&moenster_in)) == moenster_out
    {
        return 5;
    } else if moenster_in == moenster_out {
        return 6;
    } else {
        return 7;
    }
}
