pub struct Pronouns {
    subject: [String; 8]
}

impl Pronouns {
    pub fn new() -> Pronouns {
        Pronouns {
            subject: [
                "I".to_string(),
                "you".to_string(),
                "he".to_string(),
                "she".to_string(),
                "it".to_string(),
                "we".to_string(),
                "you".to_string(), 
                "they".to_string()
            ]
        }
    }
}