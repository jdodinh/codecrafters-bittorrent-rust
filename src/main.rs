use serde_json::{Value, Number};
use std::env;

// Available if you need it!
// use serde_bencode

#[cfg(test)]
mod tests;

fn decode_bencoded_value(encoded_value: &str) -> Value {
    let (decoded_value, _) = decode(encoded_value).unwrap();
    decoded_value
}

#[allow(dead_code)]
fn decode(encoded_value: &str) -> Result<(Value, usize), &'static str> {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return Ok((Value::String(string.to_string()), colon_index + 1 + number as usize));
    } else if encoded_value.chars().next().unwrap() == 'i' {
        let end_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..end_index];
        let number = number_string.parse::<i64>().unwrap();
        return Ok((Value::Number(Number::from(number)), end_index + 1));
    } else if encoded_value.chars().next().unwrap() == 'l' {
        let mut vector: Vec<Value> = Vec::new();
        let mut bytes_read: usize = 1;
        let mut string_slice = &encoded_value[1..];
        while let Ok((value, size)) = decode(string_slice) {
            vector.push(value);
            bytes_read += size;
            string_slice = &encoded_value[bytes_read..]
        }
        return Ok((Value::Array(vector), bytes_read + 1));
    } else {
        Err("invalid value to decode")
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
