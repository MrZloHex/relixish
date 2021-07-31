mod analyser;
use analyser::Analyser;

pub fn relixish(string: String) -> Analyser {
    let mut analyser = Analyser::new();
    analyser.add_string(string);

    analyser
}