use std::fmt::Debug;

#[derive(PartialEq)]
enum SampleRate {
    LOW,
    MEDIUM,
    HIGH,
    CD,
    DVD,
}

impl Debug for SampleRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LOW => write!(f, "8kHz"),
            Self::MEDIUM => write!(f, "16kHz"),
            Self::HIGH => write!(f, "32kHz"),
            Self::CD => write!(f, "44.1kHz"),
            Self::DVD => write!(f, "48kHz"),
        }
    }
}

impl SampleRate {
    pub fn get_sample_rate(&self) -> u32 {
        match self {
            SampleRate::LOW => 8000,
            SampleRate::MEDIUM => 16000,
            SampleRate::HIGH => 32000,
            SampleRate::CD => 44100,
            SampleRate::DVD => 48000,
        }
    }
}

pub struct CP16Generator {
    content: String,
    sample_rate: SampleRate,
    start_freq: u32,
    step_freq: u32,
    font_time: f64,
}

impl Default for CP16Generator {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
            sample_rate: SampleRate::LOW,
            start_freq: 1200,
            step_freq: 10,
            font_time: 1.0,
        }
    }
}

impl eframe::App for CP16Generator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CP-16 Generator");
            ui.horizontal(|ui| {
                ui.label("Content");
                ui.text_edit_singleline(&mut self.content);
            });
            ui.horizontal(|ui| {
                ui.label("Sample Rate");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.sample_rate))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sample_rate, SampleRate::LOW, "8kHz");
                        ui.selectable_value(&mut self.sample_rate, SampleRate::MEDIUM, "16kHz");
                        ui.selectable_value(&mut self.sample_rate, SampleRate::HIGH, "32kHz");
                        ui.selectable_value(&mut self.sample_rate, SampleRate::CD, "44.1kHz");
                        ui.selectable_value(&mut self.sample_rate, SampleRate::DVD, "48kHz");
                    })
            });
            ui.horizontal(|ui| {
                ui.label("Start Freq");
                ui.add(egui::DragValue::new(&mut self.start_freq).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Step Freq");
                ui.add(egui::DragValue::new(&mut self.step_freq).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Font time");
                ui.add(egui::DragValue::new(&mut self.font_time).speed(0.5));
            });
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    println!("will be played");
                }
                if ui.button("Output").clicked() {
                    println!("will be outputed to wav");
                }
            });
        });
    }
}
