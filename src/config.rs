use std::{collections::HashMap, fs};

use json::JsonValue;

use crate::errors::{error, Expect};

pub struct Config {
    pub name: String,
    pub build: HashMap<String, String>,
    pub link: String
}
impl Config {
    pub fn new(name: String, build: HashMap<String, String>, link: String) -> Self {
        Self { name, build, link }
    }
}

pub fn parse_config() -> Config {
    let j = json::parse(&fs::read_to_string(".\\nasm_proj.json").expect_np("couldn't read project config")).expect_np("couldn't parse json");
    if let JsonValue::Object(s) = j {
        let build;
        let link;
        let name;

        if let (Some(JsonValue::Object(b)), Some(JsonValue::Short(l)), Some(JsonValue::Short(n))) = (s.get("build"), s.get("link"), s.get("name")) {
            build = b;
            link = l;
            name = n;
        } else {
            error("JSON has wrong format");
        }

        let mut map = HashMap::new();
        for (x, y) in build.iter() {
            if let JsonValue::Short(v) = y {
                map.insert(x.to_string(), v.to_string());
            } else {
                continue;
            }
        }

        return Config::new(name.to_string(), map, link.to_string())
    } else {
        error("JSON has wrong format");
    }
}