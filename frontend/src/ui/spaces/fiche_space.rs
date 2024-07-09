use eframe::egui;

pub struct FicheSpace {}
impl FicheSpace {
    pub fn new() -> Self {
        FicheSpace {}
    }
}

impl eframe::App for FicheSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("My Window").show(ctx, |ui| {
                ui.label("Hello World!");
            });
        });
    }
}