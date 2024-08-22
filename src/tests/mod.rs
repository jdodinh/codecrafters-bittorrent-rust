use super::*;

#[test]
fn test_integer_decode() {
    let integer_string = "i127435439e";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(127435439 as i64)));

    let integer_string = "i-9e";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(-9 as i64)));

    let integer_string = "i0e";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(0 as i64)));

    let integer_string = "i-0e";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(0 as i64)));
}

#[test]
#[should_panic]
fn test_integer_decode_invalid() {
    let integer_string = "ie";
    let result = decode_bencoded_value(integer_string);
    assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(0 as i64)));
}
