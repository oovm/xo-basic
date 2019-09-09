use dynfmt::{Format, FormatArgs, SimpleCurlyFormat};
use lazy_static::lazy_static;
use serde_yaml::Value;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

lazy_static! {
    pub static ref EN: HashMap<&'static str, String> = { build_string_map(include_str!("../../i18n/en/basic.yml")) };
    pub static ref CHS: HashMap<&'static str, String> = { build_string_map(include_str!("../../i18n/zh-Hans/basic.yml")) };
    pub static ref CHT: HashMap<&'static str, String> = { build_string_map(include_str!("../../i18n/zh-Hant/basic.yml")) };
}

#[derive(Debug)]
pub enum Localization {
    English,
    SimplifiedChinese,
    TraditionalChinese,
}

impl Localization {
    fn get_formatter_en<A: FormatArgs>(&self, f: &str, args: A) -> Cow<str> {
        if let Some(s) = EN.get(f) {
            if let Ok(o) = SimpleCurlyFormat.format(s, args) {
                return o;
            }
        }
        Cow::from(f.to_uppercase().replace(" ", "_"))
    }

    pub fn formatter<A: Clone + FormatArgs>(&self, f: &str, args: A) -> Cow<str> {
        match self {
            Localization::English => self.get_formatter_en(f, args),
            Localization::SimplifiedChinese => {
                if let Some(s) = CHS.get(f) {
                    if let Ok(o) = SimpleCurlyFormat.format(s, args.clone()) {
                        return o;
                    }
                }
                self.get_formatter_en(f, args)
            }
            Localization::TraditionalChinese => {
                if let Some(s) = CHT.get(f) {
                    if let Ok(o) = SimpleCurlyFormat.format(s, args.clone()) {
                        return o;
                    }
                }
                self.get_formatter_en(f, args)
            }
        }
    }
}

fn build_string_map(raw: &str) -> HashMap<&'static str, String> {
    let raw: BTreeMap<String, String> = match serde_yaml::from_str(raw) {
        Ok(o) => o,
        Err(e) => panic!("{}", e),
    };
    let mut new: HashMap<&'static str, String> = HashMap::with_capacity(raw.len());

    for (k, v) in raw {
        new.insert(Box::leak(k.into_boxed_str()), v);
    }
    return new;
}

#[allow(dead_code)]
fn build_string_map2(raw: &str) -> HashMap<&'static str, String> {
    let mut map: HashMap<&'static str, String> = HashMap::new();
    match serde_yaml::to_value(raw) {
        Ok(o) => match o {
            Value::Mapping(dict) => {
                for pair in dict {
                    if let (Value::String(k), Value::String(v)) = pair {
                        map.insert(Box::leak(k.into_boxed_str()), v);
                    }
                }
            }
            _ => panic!("not map"),
        },
        Err(e) => panic!("{}", e),
    }
    return map;
}

#[test]
fn test() {
    let a = Localization::TraditionalChinese;
    println!("{}", a.formatter("game name", &[""]))
}
