use exam::Exam;
use std::io::stdin;

mod exam;
mod question;

const TRUE_STRING: &str = "i";
const FALSE_STRING: &str = "h";
const NO_ANSWER_STRING: &str = "-";

fn main() {
    clear_screen();
    let mut exam = Exam::new("kerdesek.csv");
    let mut number_of_questions = 5;
    let mut answer_points: f32 = 1.0;
    let mut minus_points: f32 = 0.5;

    loop {
        println!("Archi segéd program");
        println!("Beállított vizsgakérdések száma: {}", number_of_questions);
        println!("1 - Teszt kezdete\n2 - Vizsga beállítása\n3 - Kilépés\nAdj meg egy számot:");

        let trimmed_answer: i32 = match read_input().trim().parse() {
            Err(_x) => 0,
            Ok(x) => x,
        };

        match trimmed_answer {
            1 => start_test(&mut exam, number_of_questions, answer_points, minus_points),
            2 => modify_exam(
                &mut number_of_questions,
                &mut answer_points,
                &mut minus_points,
            ),
            3 => break,
            _ => {
                clear_screen();
            }
        }
    }
}

fn start_test(exam: &mut Exam, number_of_questions: i32, answer_points: f32, minus_points: f32) {
    clear_screen();
    let question_vector = exam.assemble_test(number_of_questions as usize);
    let mut question_counter = 1;
    let mut points: f32 = 0.0;
    for question in question_vector {
        println!("\t\t{}", question.question_type);
        let mut is_question_true = String::from("A következő állítás ");
        if question.is_questsion_true {
            is_question_true.push_str("helyes-e?");
        } else {
            is_question_true.push_str("helytelen-e?");
        }
        println!("{}", is_question_true);
        println!(
            "({}/{})  {}",
            question_counter, number_of_questions, question.question
        );

        let mut input = String::new();

        while is_valid_input(&input) {
            println!(
                "Válasz[{},{},{}]: ",
                TRUE_STRING, FALSE_STRING, NO_ANSWER_STRING
            );

            if is_valid_input(&input) {
                input = read_input();
            }
        }

        if input == TRUE_STRING && question.answer || input == FALSE_STRING && !question.answer {
            println!("A válasz helyes!\n------------\n");
            points += answer_points;
        } else if input == TRUE_STRING && !question.answer
            || input == FALSE_STRING && question.answer
        {
            points -= minus_points;
            println!("A válasz helytelen!\n------------\n");
        } else {
            println!(
                "Nem adtál választ választ! Helyes válasz: {}\n------------\n",
                if question.answer { "igaz" } else { "hamis" }
            );
        }

        question_counter += 1;
    }

    let percent: f32 = points / number_of_questions as f32 * 100.0;

    println!(
        "A teszteknek vége. Végső eredmény: {}/{} ({})\n\n------------\n\n",
        points, number_of_questions, percent
    );
}

fn is_valid_input(input: &String) -> bool {
    input != TRUE_STRING && input != FALSE_STRING && input != NO_ANSWER_STRING
}

fn read_input<'a>() -> String {
    let mut user_input: String = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Helytelen bemenet!");
    user_input.trim().to_string()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn modify_exam(number_of_questions: &mut i32, answer_points: &mut f32, minus_points: &mut f32) {
    clear_screen();
    println!("Add meg a kérdések számát:");
    let trimmed_answer: i32 = match read_input().trim().parse() {
        Ok(x) => x,
        Err(_x) => 5,
    };
    *number_of_questions = trimmed_answer;

    println!("Add meg a válaszok értékét:");
    let mut trimmed_answer: f32 = match read_input().trim().parse::<f32>() {
        Ok(x) => x,
        Err(_x) => 1.0,
    };
    *answer_points = trimmed_answer;

    println!("Add meg a helytelen válaszok értékét:");
    trimmed_answer = match read_input().trim().parse::<f32>() {
        Ok(x) => x,
        Err(_x) => 0.5,
    };
    *minus_points = trimmed_answer;

    clear_screen();
}
