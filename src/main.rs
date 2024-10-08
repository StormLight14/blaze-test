use serde::{Deserialize, Serialize};
use eframe::egui;
use std::path::PathBuf;
use std::fs;

struct ExamApp {
    current_exam: Option<Exam>,
    current_question: usize,
    choice_selections: Vec<(usize, Choice)>,
    error_message: Option<String>,
    correct_answers: Option<u32>
}

impl Default for ExamApp {
    fn default() -> Self {
        Self {
            current_exam: None,
            current_question: 0,
            choice_selections: Vec::new(),
            error_message: None,
            correct_answers: None
        }
    }
}

impl eframe::App for ExamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref error_message) = self.error_message {
                ui.label(error_message);
            }
            if let (Some(correct_answers), Some(ref current_exam)) = (self.correct_answers, &self.current_exam) {
                ui.label("Exam Submitted!");
                ui.label(format!("You got {} out of {} questions correct.", correct_answers, current_exam.question_count()));
            } else {
                if ui.button("Load Exam File").clicked() {
                    if let Some(exam_path) = rfd::FileDialog::new().add_filter("BlazeExam", &["json", "examblaze", "blaze"]).pick_file() {
                        self.load_exam(exam_path);
                    } else {
                        self.error_message = Some(String::from("Error opening exam file!"));
                    }
                }

                
                if let Some(ref mut exam_data) = self.current_exam {
                    ui.heading(&exam_data.name);
                    if self.choice_selections.len() == 0 {
                        for i in 0..exam_data.question_count() {
                            self.choice_selections.push((i, Choice::A));
                        }
                    }
                    ui.label(format!("Question {}: {}", self.current_question + 1, exam_data.questions[self.current_question].prompt));
                    for (i, choice_label) in exam_data.questions[self.current_question].choices.iter().enumerate() {
                        ui.radio_value(&mut self.choice_selections[self.current_question].1, Choice::from(i), &*choice_label);
                    }
                    ui.horizontal(|ui| {
                        if self.current_question > 0 {
                            if ui.button("Previous Question").clicked() {
                                self.current_question -= 1;
                            }
                        } 
                        if (self.current_question as i32) < (exam_data.question_count() as i32) - 1 {
                            if ui.button("Next Question").clicked() {
                                self.current_question += 1;
                            }
                        }
                    });

                    if (self.current_question as i32) == (exam_data.question_count() as i32) - 1 {
                        if ui.button("Submit Exam").clicked() {
                            self.submit_exam();
                        }
                    }
                }
            }
        });
    }
}

impl ExamApp {
    fn load_exam(&mut self, exam_path: PathBuf) {
        let exam_data = fs::read_to_string(exam_path).unwrap();
        match serde_json::from_str::<Exam>(&exam_data) {
            Ok(exam) => {
                self.current_exam = Some(exam);
            },
            Err(err) => eprintln!("{:?}", err),
        };
    }
    fn submit_exam(&mut self) {
        let mut correct_answers = 0;
        if let Some(ref exam) = self.current_exam {
            for (i, question) in exam.questions.iter().enumerate() {
                if self.choice_selections[i].1 == Choice::from(question.answer.as_str()) {
                    correct_answers += 1;
                }
            }
            self.correct_answers = Some(correct_answers);
        } else {
            self.correct_answers = None;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Exam {
    name: String,
    questions: Vec<Question>
}

impl Exam {
    fn question_count(&self) -> usize {
        self.questions.len()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    prompt: String,
    choices: Vec<String>,
    answer: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}

impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        match value {
            "A" => Choice::A,
            "B" => Choice::B,
            "C" => Choice::C,
            "D" => Choice::D,
            "E" => Choice::E,
            "F" => Choice::F,
            "G" => Choice::G,
            "H" => Choice::H,
            _ => Choice::A,
        }
    }
}

impl From<usize> for Choice {
    fn from(value: usize) -> Self {
        match value {
            0 => Choice::A,
            1 => Choice::B,
            2 => Choice::C,
            3 => Choice::D,
            4 => Choice::E,
            5 => Choice::F,
            6 => Choice::G,
            7 => Choice::H,
            _ => Choice::A,
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Exam Blaze", options, Box::new(|cc| {
        Box::<ExamApp>::default()
    }))
}

