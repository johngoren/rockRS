use std::fs;
use serde_json::{Value, from_str};
use regex::{Regex};
use std::collections::HashSet;

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
    match tag_name {
        "artist" => get_artist(),
        _ => "Other"
    }
}

fn get_artist() {

}

fn get_title() {

}

fn get_ending() {

}

fn get_body() {

}

fn get_sentence() {

}

fn get_adjective() {

}

fn get_noun() {

}

fn get_quality() {

}

fn get_description() {

}

fn get_song_name() {

}

fn get_classic_album() {

}

fn get_cliche() {

}

fn get_issue() {
    
}

fn get_grammar() -> String {
    let path = format!("./src/assets/{}", GRAMMAR_FILENAME);
    return fs::read_to_string(path).expect("Could not read file.");
}

fn get_json(grammar: &str) -> Value {
    let json:Value = from_str(&grammar).expect("Could not parse JSON.");
    json
}
