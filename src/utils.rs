use serde_json::Value;

pub fn show_json(result: Value) {
    let formatted_json = serde_json::to_string_pretty(&result)
        .unwrap_or_else(|err| format!("Format json error: {err}"));

    println!("{formatted_json}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_show_json() {
        let json_value = json!({"case": "test"});
        show_json(json_value);
    }
}
