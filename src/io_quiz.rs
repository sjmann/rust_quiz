mod question;

pub struct IoQuiz {
    pub questions: Vec<question::Question>,
}

impl IoQuiz {
    pub fn new() -> IoQuiz {
        let questions = get_questions();
        IoQuiz{ questions }
    }
}

fn get_questions() -> Vec<question::Question>
{
    let question1 = question::Question::new(String::from("Please type yes"), String::from("yes"), 0, 5,);
    let question2 = question::Question::new(String::from("Please type no"), String::from("no"), 0, 10);

    vec![question1, question2]
}
