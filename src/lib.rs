mod analyser;
use std::collections::HashMap;

use analyser::Analyser;

pub fn analys(string: String) -> HashMap<String, String> {
    let mut analyser = Analyser::new();
    analyser.add_string(string);
    analyser.analys()
}