#[derive(Clone, Debug)]
pub struct Question {
    pub answer: bool,
    pub question: String,
    pub question_type: String,
    pub is_questsion_true: bool, // ha helyeset kell bejelölni akkor true, ha a helytelenet akkor false
}

impl Question {
    pub fn new(
        answer: bool,
        question: String,
        question_type: String,
        is_questsion_true: bool,
    ) -> Question {
        Question {
            answer,
            question,
            question_type,
            is_questsion_true,
        }
    }
}
