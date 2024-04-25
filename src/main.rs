use serde::{Deserialize, Serialize};
use eframe::egui;
use std::fs;

struct ExamApp {
    current_exam: Option<Exam>
}

impl Default for ExamApp {
    fn default() -> Self {
        Self {
            current_exam: None
        }
    }
}

impl eframe::App for ExamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Exam Blaze");
            if ui.button("Load Exam").clicked() {
                self.load_exam();
            }
            
            if let Some(ref mut exam_data) = self.current_exam {
                ui.label(format!("{:?}", exam_data));
            }
        });
    }
}

impl ExamApp {
    fn load_exam(&mut self) {
        let exam_data = fs::read_to_string("exam.json").unwrap();
        match serde_json::from_str::<Exam>(&exam_data) {
            Ok(exam) => {
                println!("{:?}", exam);
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
    fn question_count(&self) -> u32 {
        self.questions.len() as u32
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    prompt: String,
    choices: Vec<String>,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Exam Blaze", options, Box::new(|cc| {
        Box::<ExamApp>::default()
    }))
}

