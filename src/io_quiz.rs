mod question;
pub mod quiz_state;

pub struct IoQuiz {
    pub questions: Vec<question::Question>,
}

impl IoQuiz {
    pub fn new() -> IoQuiz {
        let questions = get_questions();
        IoQuiz{ questions }
    }

    pub fn get_next_question(&self, state: quiz_state::QuizState) -> Option<question::Question> {
        let q = match state.is_running {
            true => Some(self.questions.get(state.question_index)),
            false => None,
        };

        state.advance(self.questions.len());
    }

    pub fn check_answer()
}

fn get_questions() -> Vec<question::Question>
{
    let question1 = question::Question::new(String::from("Please type yes"), String::from("yes"), 0, 5,);
    let question2 = question::Question::new(String::from("Please type no"), String::from("no"), 0, 10);

    vec![question1, question2]
}
