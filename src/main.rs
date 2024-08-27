use serde_json::{Value, Number, Map};
use core::panic;
use std::str;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt::Write;

// Available if you need it!
// use serde_bencode

#[cfg(test)]
mod tests;

fn decode_bencoded_value(encoded_value: Vec<u8>) -> Value {
    let (decoded_value, _) = decode(encoded_value).unwrap();
    decoded_value
}

#[allow(dead_code)]
fn decode(encoded_value: Vec<u8>) -> Result<(Value, usize), &'static str> {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.iter().next().unwrap().is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        return decode_str(encoded_value);
    } else if encoded_value.iter().next().unwrap().to_owned() == 'i' as u8 {
        return decode_int(encoded_value);
    } else if encoded_value.iter().next().unwrap().to_owned() == 'l' as u8 {
        return decode_list(encoded_value);
    } else if encoded_value.iter().next().unwrap().to_owned() == 'd' as u8 {
        return decode_dict(encoded_value);
    } else {
        Err("invalid value to decode")
    }
}

fn decode_str(encoded_value: Vec<u8>) -> Result<(Value, usize), &'static str> {
    let colon_index = encoded_value.iter().position(|c| (':' as u8).eq(c)).unwrap();
    let number_string = str::from_utf8(&encoded_value[..colon_index]).unwrap();
    let number = number_string.parse::<i64>().unwrap();
    let string = String::from_utf8_lossy(&encoded_value[colon_index + 1..colon_index + 1 + number as usize]);
    return Ok((Value::String(string.into_owned()), colon_index + 1 + number as usize));
}

fn decode_int(encoded_value: Vec<u8>) -> Result<(Value, usize), &'static str> {
    let end_index = encoded_value.iter().position(|c| ('e' as u8).eq(c)).unwrap();
    let number_string = String::from_utf8_lossy(&encoded_value[1..end_index]);
    let number = number_string.parse::<i64>().unwrap();
    return Ok((Value::Number(Number::from(number)), end_index + 1));
}

fn decode_list(encoded_value: Vec<u8>) -> Result<(Value, usize), &'static str> {
    let mut vector: Vec<Value> = Vec::new();
    let mut bytes_read: usize = 1;
    let mut string_slice = &encoded_value[1..];
    while string_slice.iter().next().unwrap().to_owned() != 'e' as u8 {
        let (value, size) = decode(string_slice.to_vec()).unwrap();
        vector.push(value);
        bytes_read += size;
        string_slice = &encoded_value[bytes_read..]
    }
    return Ok((Value::Array(vector), bytes_read + 1));
}

fn decode_dict(encoded_value: Vec<u8>) -> Result<(Value, usize), &'static str> {
    let mut bytes_read: usize = 1;
    let mut map: Map<String, Value> = Map::new();
    let mut string_slice = &encoded_value[1..];
    while string_slice.iter().next().unwrap().to_owned() != 'e' as u8 {
        let (key, offset) = decode_str(string_slice.to_vec()).unwrap();
        bytes_read += offset;
        string_slice = &string_slice[offset..];
        let (value, offset) = decode(string_slice.to_vec()).unwrap();
        bytes_read += offset;
        map.insert(key.as_str().unwrap().to_owned(), value);
        string_slice = &string_slice[offset..];
    }
    return Ok((Value::Object(map), bytes_read + 1));
}

fn fmt_torrent_details(file_contents: Vec<u8>) -> String {
    let mut buffer = String::new();
    // for (i, e) in file_contents.iter().enumerate() {
    //     println!("{}: {:?}", i, e);
    // }
    let map: Value = match decode_dict(file_contents) {
        Ok((map, _)) => map,
        Err(why) => panic!("error decoding map: {}", why),
    };
    match map.get("announce") { 
        Some(url) => writeln!(buffer, "Tracker URL: {}", url.as_str().unwrap()).unwrap(),
        None => panic!("invalid torrent file: No tracker URL found"),
    }
    match map.get("info") {
        Some(info_map) => match info_map.get("length") {
            Some(length) => writeln!(buffer, "Length: {}", length).unwrap(),
            None => panic!("invalid torrent file: No torrent length found"),
        },
        None => panic!("invalid torrent file: No tracker URL found"),
    };
    return buffer;
}


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value: Vec<u8> = Vec::from(args[2].clone());
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        let file_path = Path::new(&args[2]);
        let mut file = match File::open(&file_path) {
            Err(why) => {println!("couldn't open file: {}", why); self::panic!();},
            Ok(file) => file,
        };
        let mut file_contents: Vec<u8> = Vec::new();
        match file.read_to_end(&mut file_contents) {
            Err(why) => {println!("failed to read file contents: {}", why); self::panic!()},
            Ok(_) => (),
        }
        println!("{}", fmt_torrent_details(file_contents));
    } else {
        println!("unknown command: {}", args[1])
    }
}
