use anyhow::Result;
use egui::Context;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, SyncSender};

#[derive(Clone)]
pub struct App {
    pub text: String,
    pub tx: Arc<SyncSender<String>>,
    pub rx: Arc<RefCell<Receiver<String>>>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(10);

        Self {
            text: "hello world".to_string(),
            tx: Arc::new(tx),
            rx: Arc::new(RefCell::new(rx)),
        }
    }
}

impl App {
    pub fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click each year").clicked() {
                self.fetch_data();
            }

            ui.separator();

            if let Ok(text) = self.rx.borrow_mut().try_recv() {
                self.text = text;
                ui.label(&self.text);
            }

            ui.label(&self.text);
        });
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
