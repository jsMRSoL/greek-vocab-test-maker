use serde::Deserialize;

#[derive(Debug)]
pub struct Question {
    pub greek: String,
    pub answers: Vec<AnswerOption>,
}

#[derive(Debug)]
pub struct AnswerOption {
    pub mark: u8,
    pub answer: String,
    pub feedback: String,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub greek: String,
    #[serde(rename = "Part of Speech")]
    pub part_of_speech: String,
    pub english: String,
}
