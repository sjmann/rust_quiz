pub struct QuizState {
    pub question_index: usize,
    pub score: u8,
    pub is_running: bool,
}

impl QuizState {
    pub fn new() -> QuizState {
        QuizState { question_index: 0, score: 0, is_running: true }
    }

    pub fn advance(&mut self) {
        self.question_index += 1;
    }
}
