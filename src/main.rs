use json;

fn main() {
    println!("-- Parsing valid JSON data --");
    load_json(
        r#"
{
    "name": "Fido",
    "hasSpots": false,
    "items": {
        "snacks": [
            "biscuit",
            "beef chew"
        ]
    }
}
"#,
    );

    println!("-- Parsing invalid JSON data --");
    load_json(
        r#"
    "name": "Fido",
    "hasSpots": false,
    "items": {
        "snacks": [
            "biscuit",
            "beef chew"
        ]
    }
}
"#,
    );
}

fn load_json(json_string: &str) {
    let _data = json::parse(json_string).unwrap_or_else(|error| {
        println!("\tError parsing JSON: {}", error);
        return json::JsonValue::new_object();
    });

    println!("JSON data retrieved\n\t{}", _data);
}
