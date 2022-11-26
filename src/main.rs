mod translator;

use eframe::egui;

const VERSION: &str = "1.4";

struct Frame {
    input: String,
    output: String,
    start: bool
}

impl Default for Frame {
    fn default() -> Self {
        Self { input: "".to_owned(), output: String::new(), start: true }
    }
}


impl eframe::App for Frame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { input, output, start} = self;
        let mut y = 0.;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Version {VERSION}"));
            let r1 = ui.text_edit_multiline(input);
            let r2 = ui.text_edit_multiline(output);

            if *start {
                r1.request_focus();
                *start = false;
            }

            let events = ui.input().events.clone();
            for event in &events {
                match event {
                    
                    egui::Event::Key{key, pressed, modifiers: _} => {
                        if *key == eframe::egui::Key::Enter && *pressed && input.len() >= 5 && r1.has_focus() {
                            let result = translator::translate(input[..2].to_string(), input[2..4].to_string(), input[4..].to_string());
                            *output = result.unwrap();
                            *input = input.replace("\n", "");
                        }
                        else if *key == eframe::egui::Key::Escape && *pressed {
                            std::process::exit(0);
                        }
                    },
                    _ => {}
                }
            }

            y = r1.rect.height() + r2.rect.height();
        });

        let mut actual_size = ctx.used_size();
        actual_size.y = y + 32.;
        _frame.set_window_size(actual_size);
    }
}

fn main() {
    let app = Frame::default();
    let options = eframe::NativeOptions {
        always_on_top: true,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_pos: Option::from(egui::Pos2::new(1650., 30.)),
        initial_window_size: Option::from(egui::Vec2::new(250., 135.)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::default(),
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
    };
    eframe::run_native("Quick Translator", options, Box::new(|_| Box::new(app)));
}
