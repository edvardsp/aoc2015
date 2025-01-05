use serde_json::{Map, Value};

advent_of_code::solution!(12);

fn is_red(object: &Map<String, Value>) -> bool {
    object.values().any(|value| value == "red")
}

fn sum_numbers(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
        Value::Object(m) => m.values().map(sum_numbers).sum(),
        Value::String(_) | Value::Bool(_) | Value::Null => 0,
    }
}

fn sum_numbers_without_red(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numbers_without_red).sum(),
        Value::Object(m) if is_red(m) => 0,
        Value::Object(m) => m.values().map(sum_numbers_without_red).sum(),
        Value::String(_) | Value::Bool(_) | Value::Null => 0,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let json: Value = serde_json::from_str(input).unwrap();
    Some(sum_numbers(&json))
}

pub fn part_two(input: &str) -> Option<i64> {
    let json: Value = serde_json::from_str(input).unwrap();
    Some(sum_numbers_without_red(&json))
}
