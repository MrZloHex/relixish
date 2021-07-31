use std::collections::HashMap;

pub struct Analyser {
    string: String
}

impl Analyser {
    pub fn new() -> Analyser {
        Analyser {
            string: String::new()
        }
    }

    pub fn add_string(&mut self, string: String) {
        self.string = string;
    }

    pub fn analys(&mut self) -> HashMap<String, String>{
        let data: HashMap<String, String> = HashMap::new();


        data
    }
}