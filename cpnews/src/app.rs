use super::about;
use super::theme;
use super::tr::tr;
use anyhow::Result;
use egui::Context;
use egui::{Align, FontId, ImageButton, Layout, Pos2, RichText, ScrollArea, TextStyle, Ui, Window};
use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;

#[derive(Clone, Debug)]
enum MsgType {
    Info,
    Warn,
    Success,
    Danger,
}

impl Default for MsgType {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Clone, Debug, Default)]
struct MsgSpec {
    show: bool,
    msg: String,
    msg_type: MsgType,
}

#[derive(Clone, Debug, Default)]
struct NewsItem {
    pub title: String,
    pub summary: String,
    pub date: String,
    pub link: String,
}

#[derive(Clone)]
pub struct App {
    pub about: String,
    pub is_cn: bool,
    pub msg_spec: MsgSpec,
    pub tx: Arc<SyncSender<NewsItem>>,
    pub rx: Arc<RefCell<Receiver<NewsItem>>>,
    pub news_items: Vec<NewsItem>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(1024);

        Self {
            is_cn: true,
            msg_spec: Default::default(),
            about: about::about(),
            tx: Arc::new(tx),
            rx: Arc::new(RefCell::new(rx)),
            news_items: vec![],
        }
    }
}

impl App {
    pub fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.header(ui);
            self.news_list(ui);

            self.update_data();
        });

        self.popup_message(ctx);
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new(tr(self.is_cn, "加密新闻")).color(theme::BRAND_COLOR));
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
                    .add(ImageButton::new(lang_icon.id(), theme::ICON_SIZE).frame(false))
                    .clicked()
                {
                    self.is_cn = !self.is_cn;
                }

                if ui
                    .add(ImageButton::new(refresh_icon.id(), theme::ICON_SIZE).frame(false))
                    .clicked()
                {
                    // self.show_message("hello world".to_string(), MsgType::Success);
                    self.fetch_data();
                }
            });
        });

        ui.separator();
    }

    fn news_list(&mut self, ui: &mut Ui) {
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = self.news_items.len();

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, num_rows, |ui, row_range| {
                for row in row_range {
                    self.show_news_item(ui, &self.news_items[row]);
                }
            });
    }

    fn show_news_item(&self, ui: &mut Ui, item: &NewsItem) {
        ui.vertical(|ui| {
            ui.label(
                RichText::new(&item.title)
                    .color(theme::NEWS_TITLE_COLOR)
                    .font(FontId::proportional(theme::NEWS_TITLE_FONT_SIZE)),
            );

            ui.add_space(theme::SPACING);

            ui.label(&item.summary);

            ui.add_space(theme::SPACING);

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.label(RichText::new(&item.date).color(theme::LIGHT_COLOR));

                if !item.link.is_empty() {
                    ui.add_space(theme::SPACING);

                    ui.hyperlink_to(
                        RichText::new(tr(self.is_cn, "原文链接")).color(theme::LIGHT_COLOR),
                        &item.link,
                    );
                }
            });

            ui.add_space(theme::SPACING);
        });
    }

    // TODO
    fn update_data(&mut self) {
        if let Ok(item) = self.rx.borrow_mut().try_recv() {
            self.news_items.push(item);
        }
    }

    fn fetch_data(&mut self) {
        let tx = self.tx.clone();

        std::thread::spawn(move || {
            let _ = tx.try_send(NewsItem {
                title: "在一幅刻".to_string(),
                summary: "在一幅刻制于南宋年间的《平江图》展板前，习近平仔细察看。沿着石板路，他走进古街巷。历经岁月沧桑，如今的姑苏古城与《平江图》里的古苏州整体布局基本一致。".to_string(),
                date: "2023-12-9 12:09:08".to_string(),
                link: "http://google.com".to_string(),
            });
        });

        // move || match reqwest::blocking::get("https://httpbin.org/ip") {
        //     Err(e) => {
        //         let _ = tx.try_send(format!("{e:?}"));
        //     }
        //     Ok(resp) => match resp.text() {
        //         Err(e) => {
        //             let _ = tx.try_send(format!("{e:?}"));
        //         }
        //         Ok(text) => {
        //             let _ = tx.try_send(text);
        //         }
        //     },
        // },
    }

    fn popup_message(&mut self, ctx: &Context) {
        let mut is_show = self.msg_spec.show;
        Window::new("popup-message")
            .title_bar(false)
            .open(&mut is_show)
            .collapsible(false)
            .auto_sized()
            .constrain(true)
            .interactable(false)
            .fixed_pos(Pos2::new(10.0, 60.0))
            .show(ctx, |ui| {
                ui.label(&self.msg_spec.msg);
            });
    }

    fn show_message(&mut self, msg: String, msg_type: MsgType) {
        self.msg_spec.show = true;
        self.msg_spec.msg = msg;
        self.msg_spec.msg_type = msg_type;
    }
}

#[allow(unused)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
