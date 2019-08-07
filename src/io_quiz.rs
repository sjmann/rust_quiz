use std::io;

mod question;
pub mod quiz_state;


pub struct IoQuiz {
    state: quiz_state::QuizState,
    pub questions: Vec<question::Question>,
}

impl IoQuiz {
    pub fn new() -> IoQuiz {
        let questions = get_questions();
        let state = quiz_state::QuizState::new();
        IoQuiz{ questions, state }
    }

    pub fn is_running(&self) -> bool {
        self.state.is_running
    }

    pub fn ask_next(&mut self) {
        match self.get_next_question() {
            Some(question) => {
                println!("{}", question.text);

                let user_answer = poll_for_answer();
                match question.check_answer(&user_answer) {
                    true => {
                        println!("NICE!!");
                        self.state.score += question.score;
                    },
                    false => println!("AWFUL!"),
                };
            },
            None => {
                self.state.is_running = false;
                println!("QUIZ COMPLETE!");
                println!("You Scored: {}", self.state.score);
            },
        };

        self.state.advance();
    }

    fn get_next_question(&self) -> Option<&question::Question> {
        self.questions.get(self.state.question_index)
    }
}

fn get_questions() -> Vec<question::Question>
{
    let question1 = question::Question::new(String::from("Please type yes"), String::from("yes"), 0, 5,);
    let question2 = question::Question::new(String::from("Please type no"), String::from("no"), 0, 10);

    vec![question1, question2]
}

fn poll_for_answer() -> String {
    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line!");

    String::from(answer.trim())
}
