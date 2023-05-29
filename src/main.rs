// use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::fs::File;

fn untyped_example(data: &str) -> Value {
    serde_json::from_str(data).unwrap()
}
fn main() {
    let data = r#"
	{
		"name": "John Doe",
		"age": 43,
		"phones": [
			"+44 1234567",
			"+44 2345678"
		]
	}"#;
    let v = untyped_example(data);
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
}
