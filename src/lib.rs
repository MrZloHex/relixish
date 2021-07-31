mod language_items;

mod analyser;
use std::collections::HashMap;

use analyser::Analyser;

pub fn analys_string(string: String) -> HashMap<String, String> {
    println!("Analysing string '{}'", string);

    let mut analyser = Analyser::new();
    analyser.add_string(string);
    let output = analyser.analys();
    output
}