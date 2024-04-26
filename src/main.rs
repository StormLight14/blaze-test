use serde::{Deserialize, Serialize};
use eframe::egui;
use std::path::PathBuf;
use std::fs;

struct ExamApp {
    current_exam: Option<Exam>,
    current_question: usize,
    choice_selections: Vec<(usize, Choice)>,
}

impl Default for ExamApp {
    fn default() -> Self {
        Self {
            current_exam: None,
            current_question: 0,
            choice_selections: Vec::new()
        }
    }
}

impl eframe::App for ExamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Load Exam File").clicked() {
                if let Some(exam_path) = rfd::FileDialog::new().add_filter("exam blaze test", &["json", "blaze"]).pick_file() {
                    self.load_exam(exam_path);
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

