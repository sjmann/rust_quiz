use std::io;

fn main() {
    let questions = get_questions();
    let mut score = 0;

    let chapter: u8 = match ask_question("What chapter have you read?").parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Chapter: {}", chapter);

    for question in questions.iter() {
        let answer = ask_question(&question.text[..]);

        match question.check_answer(&answer[..]) {
            true => {
                println!("NICE");
                score += question.score;
            },
            false => println!("WRONG")
        };
    }

    println!("You scored: {}", score);
}

struct Question {
    text: String,
    correct_answer: String,
    chapter: u8,
    score: u8,
}

impl Question {
    fn check_answer(&self, answer: &str) -> bool {
        answer == self.correct_answer
    }

    fn new(text: String, correct_answer: String, chapter: u8, score: u8) -> Question {
        Question {
            text,
            correct_answer,
            chapter,
            score,
        }
    }
}

fn ask_question(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line!");

    String::from(answer.trim())
}

fn get_questions() -> [Question; 2]
{
    let question1 = Question::new(String::from("Please type yes"), String::from("yes"), 0, 5,);
    let question2 = Question::new(String::from("Please type no"), String::from("no"), 0, 10);

    [question1, question2]
}
