use egui::Context;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("desktop egui demo")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{} egui on GitHub", GITHUB),
                    "https://github.com/emilk/egui",
                );
                ui.hyperlink_to(
                    format!("{} @ernerfeldt", TWITTER),
                    "https://twitter.com/ernerfeldt",
                );

                ui.separator();
            });

        self.show_windows(ctx);
    }

    // Show the open windows.
    fn show_windows(&mut self, _ctx: &Context) {}
}

#[allow(unused)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
