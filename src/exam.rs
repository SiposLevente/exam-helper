use crate::question::*;
use rand::Rng;
use std::fs;

pub struct Exam {
    questions: Vec<Question>,
}

impl Exam {
    pub fn new(questions: &str) -> Exam {
        Exam {
            questions: Exam::read_data_into_vector(questions),
        }
    }

    pub fn read_data_into_vector(file: &str) -> Vec<Question> {
        let mut question_vector: Vec<Question> = vec![];
        let content: String = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(x) => panic!("{}", x),
        };

        for line in content.split("\n") {
            let data: Vec<&str> = line.split(";").collect();
            if data.len() == 4 {
                let mut is_question_true = true;
                let mut answer = true;
                if data[3] == "helytelen" || data[3] == "helytelen\r" {
                    is_question_true = false;
                }
                if data[0] == "h" {
                    answer = false;
                }

                question_vector.push(Question::new(
                    answer,
                    data[1].to_string(),
                    data[2].to_string(),
                    is_question_true,
                ));
            }
        }

        question_vector
    }

    pub fn assemble_test(&mut self, question_number: usize) -> Vec<Question> {
        let mut rng = rand::thread_rng();
        let mut questions: Vec<Question> = vec![];
        let mut question_numbers: Vec<usize> = vec![];
        for _i in 0..question_number {
            let mut question_number = rng.gen_range(0..self.questions.len());
            
            while question_numbers.contains(&question_number) {
                question_number = rng.gen_range(0..self.questions.len());
            }
            question_numbers.push(question_number);
            questions.push(self.questions[question_number].clone());
        }
        questions
    }
}
