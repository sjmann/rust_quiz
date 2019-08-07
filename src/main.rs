use std::io;

mod io_quiz;

fn main() {
    let mut quiz = io_quiz::IoQuiz::new();

    let chapter: u8 = match ask_question("What chapter have you read?").parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Chapter: {}", chapter);

    while quiz.is_running() {
        quiz.ask_next();
    }
}

fn ask_question(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line!");

    String::from(answer.trim())
}
