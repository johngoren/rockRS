use std::fs;
use serde_json::{Value, from_str};
use regex::{Regex};
use std::collections::HashSet;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};


// #[macro_use(lazy_static)]
// extern crate lazy_static;


static GRAMMAR_FILENAME:&str = "grammar.json";

fn main() {
    let grammar = get_grammar();
    let populated_text = populate(grammar);
    println!("{}", populated_text);
}

fn populate(grammar: String) -> String {
    let json = get_json(grammar.as_str());
    let root = &json["root"];
    let populated = recursively_fill_in_fields(root);
    return populated;
}

fn recursively_fill_in_fields(root: &Value) -> String {
    let json_top_level = &root["value"];
    let raw_text = json_top_level.to_string();

    let mut new_text = String::from(&raw_text);

    loop {
        let mut matches = find_tags(&raw_text);
        for mat in matches {
            new_text = get_text_replacing_tag(new_text, &mat);
        }
        let next_text = String::from(&new_text);
        matches = find_tags(&next_text);
        let num_matches = matches.iter().count();
        if num_matches == 0 {
            break;
        }
    }

    new_text
}

fn find_tags(text: &str) -> HashSet<&str> {
    let re = Regex::new(r"\{(.*?)}").unwrap();  // TODO: Extract to static constant
    return re.find_iter(&text).map(|mat| mat.as_str()).collect();
}

fn get_text_replacing_tag(old_text: String, tag: &str) -> String {
    let mad_lib = get_mad_lib_text_for_tag(tag);
    return old_text.replace(tag, &mad_lib);
}

fn get_mad_lib_text_for_tag(tag_name: &str) -> &str {
    return lookup_mad_lib_text_for_tag(tag_name);
}

fn lookup_mad_lib_text_for_tag(tag: &str) -> &str {
    let last_char = tag.chars().count() -1;
    let tag_name = &tag[1..last_char];
    return get_text_from_tag(tag_name);
}

fn get_text_from_tag(tag_name: &str) -> &str {
    let grammar = get_grammar();
    let json = get_json(grammar.as_str());
    let parent = &json[tag_name];
    let meta = &parent["meta"];
    let lookup_type = meta["type"].as_str().unwrap();
    let metric = &meta["metric"];
    let special = &meta["special"];

    return match lookup_type {
        "WEIGHTED" => {
            get_weighted_text(tag_name)
        },
        "PRESET" => get_preset_text(tag_name),
        "UNIQUE" => get_random_text(tag_name),
        _ => { return "" }
    }
}

fn get_score(tag_name: &str) -> Option<i8> {
    return Some(0);
}

fn get_random_text(tag: &str) -> &str {
    let grammar = get_grammar();
    let json = get_json(grammar.as_str());
    let values = &json[tag]["value"];
    let mut rng = rand::thread_rng();
    let random_index = Uniform::from(0, values.count());
    return values[random_index];
}

// TODO: Understand weighted distribution
fn get_weighted_text(tag: &str)  -> &str {
    let grammar = get_grammar();
    let json = get_json(grammar.as_str());
    let values = &json[tag]["value"].expect("Could not find values");
    let mut rng = rand::thread_rng();
    let random_index = Uniform::from(rng);
    return values[random_index];
}

fn get_preset_text(tag: &str) -> &str {
    "TODO"
}




fn get_grammar() -> String {
    let path = format!("./src/assets/{}", GRAMMAR_FILENAME);
    return fs::read_to_string(path).expect("Could not read file.");
}

fn get_json(grammar: &str) -> Value {
    let json:Value = from_str(&grammar).expect("Could not parse JSON.");
    json
}
