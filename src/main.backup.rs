use std::io;
mod io_quiz;
use io_quiz::quiz_state;

fn main() {
    let quiz = io_quiz::IoQuiz::new();
    let mut state = quiz_state::QuizState::new();

    let chapter: u8 = match ask_question("What chapter have you read?").parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Chapter: {}", chapter);

    for question in &quiz.questions {
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

fn ask_question(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line!");

    String::from(answer.trim())
}
