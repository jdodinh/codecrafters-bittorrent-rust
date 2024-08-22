use super::*;

#[test]
fn test_integer_decode() {
    let integer_string = "i127435439e";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(127435439 as i64)));
}
