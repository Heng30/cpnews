use anyhow::Result;
use egui::Context;
use egui::{Align, Button, Layout, RichText, ScrollArea, TextStyle, Ui};
use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;

use super::theme;
use super::tr::tr;

#[derive(Clone)]
pub struct App {
    pub text: String,
    pub is_cn: bool,
    pub tx: Arc<SyncSender<String>>,
    pub rx: Arc<RefCell<Receiver<String>>>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(10);

        Self {
            is_cn: true,
            text: "hello world".to_string(),
            tx: Arc::new(tx),
            rx: Arc::new(RefCell::new(rx)),
        }
    }
}

impl App {
    pub fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.header(ui);
            ui.separator();
            self.news_list(ui);

            self.update_data();
        });
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new(tr(self.is_cn, "Odaily 新闻")).color(theme::BRAND_COLOR));
            ui.add_space(theme::SPACING);

            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                let refresh_icon = ui.ctx().load_texture(
                    "refresh-icon",
                    theme::load_image_from_memory(theme::REFRESH_ICON),
                    Default::default(),
                );

                let lang_icon = ui.ctx().load_texture(
                    "lang-icon",
                    theme::load_image_from_memory(theme::LANG_ICON),
                    Default::default(),
                );

                if ui
                    .add(Button::image_and_text(
                        lang_icon.id(),
                        theme::ICON_SIZE,
                        tr(self.is_cn, "中文"),
                    ))
                    .clicked()
                {
                    self.is_cn = !self.is_cn;
                }

                if ui
                    .add(Button::image_and_text(
                        refresh_icon.id(),
                        theme::ICON_SIZE,
                        tr(self.is_cn, "刷新"),
                    ))
                    .clicked()
                {
                    // TODO
                }
            });
        });
    }

    fn news_list(&mut self, ui: &mut Ui) {
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = 10_000;
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, num_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("This is row {}/{}", row + 1, num_rows);
                    ui.label(text);
                }
            });
    }

    // TODO
    fn update_data(&mut self) {
        if let Ok(text) = self.rx.borrow_mut().try_recv() {
            self.text = text;
        }
    }

    fn fetch_data(&mut self) {
        let tx = self.tx.clone();

        std::thread::spawn(
            move || match reqwest::blocking::get("https://httpbin.org/ip") {
                Err(e) => {
                    let _ = tx.try_send(format!("{e:?}"));
                }
                Ok(resp) => match resp.text() {
                    Err(e) => {
                        let _ = tx.try_send(format!("{e:?}"));
                    }
                    Ok(text) => {
                        let _ = tx.try_send(text);
                    }
                },
            },
        );
    }
}

#[allow(unused)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
