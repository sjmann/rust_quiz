pub struct QuizState {
    pub question_index: u8,
    pub score: u8,
    pub is_running: bool,
}

impl QuizState {
    pub fn new() -> QuizState {
        QuizState { question_index: 0, score: 0, is_running: true }
    }

    pub fn advance(&self, quiz_length: usize) {
        self.question_index += 1;

        if self.question_index >= quiz_length {
            self.is_running = false;
        }
    }
}
