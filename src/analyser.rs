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
}