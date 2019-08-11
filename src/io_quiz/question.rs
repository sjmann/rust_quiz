pub struct Question {
    text: String,
    correct_answer: String,
    chapter: u8,
    score: u8,
}

impl Question {
    pub fn check_answer(&self, answer: &str) -> bool {
        answer == self.correct_answer
    }

    pub fn new(text: String, correct_answer: String, chapter: u8, score: u8) -> Question {
        Question {
            text,
            correct_answer,
            chapter,
            score,
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn score(&self) -> u8 {
        self.score
    }
}
