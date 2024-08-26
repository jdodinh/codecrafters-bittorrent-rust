use super::*;

#[test]
fn test_string_decode() {
    let string = "5:hello";
    let result = decode_bencoded_value(string);
    assert_eq!(result, Value::String("hello".to_owned()));

    let long_string = "15:hellohellohello";
    let result = decode_bencoded_value(long_string);
    assert_eq!(result, Value::String("hellohellohello".to_owned()));
}

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

#[test]
fn test_list_decode() {
    let list_string = "l5:helloi52ee";
    let result = decode_bencoded_value(list_string);
    assert_eq!(result, Value::Array(Vec::from([Value::String("hello".to_owned()), Value::Number(Number::from(52))])));

    let list_string = "l15:hellohellohelloi52ee";
    let result = decode_bencoded_value(list_string);
    assert_eq!(result, Value::Array(Vec::from([Value::String("hellohellohello".to_owned()), Value::Number(Number::from(52))])));

    let list_string = "l5:helloi52el5:helloi52eee";
    let result = decode_bencoded_value(list_string);
    assert_eq!(result, Value::Array(Vec::from([Value::String("hello".to_owned()), Value::Number(Number::from(52)), Value::Array(Vec::from([Value::String("hello".to_owned()), Value::Number(Number::from(52))]))])));

    let list_string = "llllll5:helloeeeeee";
    let result = decode_bencoded_value(list_string);
    let expected = Value::Array(Vec::from([Value::Array(Vec::from([Value::Array(Vec::from([Value::Array(Vec::from([Value::Array(Vec::from([Value::Array(Vec::from([Value::String("hello".to_owned())]))]))]))]))]))]));
    assert_eq!(result, expected);
}

#[test]
fn test_dict_decode() {
    let dict_string = "d3:foo3:bar5:helloi52ee";
    let result = decode_bencoded_value(dict_string);
    let mut expected = serde_json::Map::new();
    expected.insert("foo".to_owned(), Value::String("bar".to_owned()));
    expected.insert("hello".to_owned(), Value::Number(Number::from(52)));
    let expected: Value = expected.into();
    assert_eq!(result, expected);
}


