use std::collections::HashMap;
use super::language_items::pronouns::Pronouns;

pub struct Analyser {
    string: String,

    pronouns: Pronouns
}

impl Analyser {
    pub fn new() -> Analyser {
        Analyser {
            string: String::new(),

            pronouns: Pronouns::new()
        }
    }

    pub fn add_string(&mut self, string: String) {
        self.string = string;
    }

    pub fn analys(&mut self) -> HashMap<String, String>{
        let mut data: HashMap<String, String> = HashMap::new();
        
        let words = self.string.split_whitespace();
        for word in words {
            println!("{}", word)
        }



        data
    }
}