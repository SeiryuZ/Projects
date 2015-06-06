use std::ascii::AsciiExt;

#[derive(Debug)]
enum ShiftDirection {
    Left,
    Right
}

fn encode_caesar(string: &mut String, key: u8, direction: ShiftDirection) {

    let mut bytes = (*string).clone().into_bytes();

    for byte in &mut bytes {
        if *byte == 32 {
            continue;
        }

        match direction {
            ShiftDirection::Right => {
                *byte = *byte - 65; // A, unicode 65 as base
                *byte = *byte + key;
                *byte = *byte % 26; // Make sure it roll over
                *byte = *byte + 65; // Return to correct one
            },
            ShiftDirection::Left => {
                *byte = *byte - 65;

                if *byte < key {
                    *byte = key - *byte;
                    *byte = 26 - *byte;
                }
                else {
                    *byte = *byte - key;
                }
                *byte = *byte + 65;
            }
        }
    }
    *string = String::from_utf8(bytes).unwrap();
}

fn decode_caesar(string: &mut String, key: u8, direction: ShiftDirection) {
    match direction {
        ShiftDirection::Right => encode_caesar(string, key, ShiftDirection::Left),
        ShiftDirection::Left => encode_caesar(string, key, ShiftDirection::Right)
    }
}


fn main() {

    let mut input = "test abcdef".to_string().to_ascii_uppercase();

    println!("Original String: {}", input);

    encode_caesar(&mut input, 1, ShiftDirection::Left);
    println!("encoded left 1: {}", input);

    decode_caesar(&mut input, 1, ShiftDirection::Left);
    println!("decoded left 1: {}", input);

    encode_caesar(&mut input, 23, ShiftDirection::Right);
    println!("encoded right 23: {}", input);

    decode_caesar(&mut input, 23, ShiftDirection::Right);
    println!("decoded right 23: {}", input);
}
