mod analyser;
use analyser::Analyser;

pub fn analys(string: String) {
    let mut analyser = Analyser::new();
    analyser.add_string(string);
    analyser.analys();
}