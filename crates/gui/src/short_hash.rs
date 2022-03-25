// shorthash
// (c) 2013 Bibig

// https://github.com/bibig/node-shorthash
// shorthash may be freely distributed under the MIT license.
// Source: https://github.com/bibig/node-shorthash/blob/master/shorthash.js

pub fn unique_hash(text: &str) -> String {
    let id = binary_transfer(bitwise(text), 61);

    id.replace("-", "Z")
}

fn bitwise(string: &str) -> i32 {
    let mut hash = 0;
    if string.len() == 0 {
        return hash;
    };

    for b in string.as_bytes() {
        // Cast to i128 to prevent subtraction overflow
        hash = ((hash << 5) as i128 - hash as i128) as i32 + (*b as i32);
    }

    hash
}

// 10进制转化成62进制以内的进制
// convert 10 binary to customized binary, max is 62
fn binary_transfer(mut integer: i32, binary: i32) -> String {
    let mut stack = vec![];
    let mut num;
    let mut result = String::new();
    let sign = if integer < 0 { "-" } else { "" };

    let table = |num| {
        let t: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();

        t[num as usize]
    };

    integer = integer.abs();

    while integer >= binary {
        num = integer % binary;
        integer = (integer / binary) as i32;
        stack.push(table(num));
    }

    if integer > 0 {
        stack.push(table(integer));
    }

    for i in 0..stack.len() {
        result.push(stack[i]);
    }

    sign.to_string() + &result
}
