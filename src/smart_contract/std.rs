pub mod string {
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
}

pub mod math {
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    pub fn subtract(a: i64, b: i64) -> i64 {
        a - b
    }

    pub fn multiply(a: i64, b: i64) -> i64 {
        a * b
    }

    pub fn divide(a: i64, b: i64) -> Result<i64, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}

pub mod data_structures {
    use std::collections::HashMap;

    pub fn new_map() -> HashMap<String, String> {
        HashMap::new()
    }

    pub fn insert(map: &mut HashMap<String, String>, key: String, value: String) {
        map.insert(key, value);
    }

    pub fn get(map: &HashMap<String, String>, key: &str) -> Option<&String> {
        map.get(key)
    }
}
