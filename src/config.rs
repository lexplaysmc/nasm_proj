use std::{borrow::Cow, collections::HashMap, fs};

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

        if let (Some(JsonValue::Object(b)), Some(l), Some(n)) = (s.get("build"), json_to_string(s.get("link").expect_np("JSON has wrong format")), json_to_string(s.get("name").expect_np("JSON has wrong format"))) {
            build = b;
            link = l;
            name = n;
        } else {
            error("JSON has wrong format");
        }

        let mut map = HashMap::new();
        for (x, y) in build.iter() {
            if let Some(v) = json_to_string(y) {
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

fn json_to_string(j: &JsonValue) -> Option<Cow<String>> {
    match j {
        JsonValue::Short(x) => {
            Some(Cow::Owned(x.to_string()))
        }
        JsonValue::String(x) => {
            Some(Cow::Borrowed(x))
        }
        _ => {
            None
        }
    }
}