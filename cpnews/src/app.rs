use super::{about, news, news::NewsItem, theme, tr::tr, util};
use egui::{
    containers::scroll_area::ScrollBarVisibility, containers::Frame, Align, Button, Color32,
    Context, FontId, ImageButton, Layout, Pos2, RichText, ScrollArea, Stroke, TextStyle,
    TextureHandle, Ui, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;

#[allow(unused)]
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
    msg: String,
    msg_type: MsgType,
    timestamp: i64,
}

#[derive(Clone, Debug)]
enum ChannelItem {
    ErrMsg(String),
    NewsItems(Vec<NewsItem>),
}

#[derive(Clone)]
pub struct App {
    pub is_cn: bool,
    pub is_fetching: bool,
    pub is_scroll_to_top: bool,
    pub news_items: Vec<NewsItem>,

    pub about: String,
    msg_spec: MsgSpec,

    tx: Arc<SyncSender<ChannelItem>>,
    rx: Rc<RefCell<Receiver<ChannelItem>>>,

    pub brand_icon: Option<TextureHandle>,
    pub refresh_icon: Option<TextureHandle>,
    pub language_icon: Option<TextureHandle>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(10);

        Self {
            is_cn: true,
            is_fetching: false,
            is_scroll_to_top: false,
            news_items: vec![],

            about: about::about(),
            msg_spec: Default::default(),

            tx: Arc::new(tx),
            rx: Rc::new(RefCell::new(rx)),

            brand_icon: None,
            refresh_icon: None,
            language_icon: None,
        }
    }
}

impl App {
    pub fn init(&mut self, ctx: &Context) {
        self.fetch_data();

        self.brand_icon = Some(ctx.load_texture(
            "brand-icon",
            theme::load_image_from_memory(theme::BRAND_ICON),
            Default::default(),
        ));

        self.refresh_icon = Some(ctx.load_texture(
            "refresh-icon",
            theme::load_image_from_memory(theme::REFRESH_ICON),
            Default::default(),
        ));

        self.language_icon = Some(ctx.load_texture(
            "language-icon",
            theme::load_image_from_memory(theme::LANGUAGE_ICON),
            Default::default(),
        ));
    }

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
            ui.image(&self.brand_icon.clone().unwrap(), theme::ICON_SIZE);
            ui.heading(RichText::new(tr(self.is_cn, "加密新闻")).color(theme::BRAND_COLOR));

            // double-clicked-area to scroll to top
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    Frame::none()
                        // .fill(theme::WARN_COLOR)
                        // .stroke(Stroke {
                        //     width: 1.0,
                        //     color: Color32::BLACK,
                        // })
                        .show(ui, |ui| {
                            let btn = Button::new("").frame(false);
                            if ui.add(btn).double_clicked() {
                                self.is_scroll_to_top = true;
                            }
                        });
                },
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(ImageButton::new(
                            self.language_icon.clone().unwrap().id(),
                            theme::ICON_SIZE).frame(false))
                    .clicked()
                {
                    self.is_cn = !self.is_cn;
                }

                if ui
                    .add(ImageButton::new(
                            self.refresh_icon.clone().unwrap().id(),
                            theme::ICON_SIZE).frame(false))
                    .clicked()
                {
                    self.fetch_data();
                }

                if self.is_fetching {
                    ui.label(
                        RichText::new(&tr(self.is_cn, "正在刷新")).color(theme::NEWS_TITLE_COLOR),
                    );
                }
            });
        });

        ui.separator();
    }

    fn news_list(&mut self, ui: &mut Ui) {
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = self.news_items.len();

        let mut sarea = ScrollArea::vertical()
            .auto_shrink([false, false])
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible);

        if self.is_scroll_to_top {
            sarea = sarea.vertical_scroll_offset(0.0);
            self.is_scroll_to_top = false;
        }

        sarea.show_rows(ui, row_height, num_rows, |ui, row_range| {
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

    fn update_data(&mut self) {
        let rx = self.rx.clone();

        if let Ok(item) = rx.borrow_mut().try_recv() {
            match item {
                ChannelItem::ErrMsg(msg) => self.show_message(msg, MsgType::Warn),
                ChannelItem::NewsItems(items) => {
                    if !items.is_empty() {
                        self.news_items = items;
                    }
                }
            }

            self.is_fetching = false;
        };
    }

    fn fetch_data(&mut self) {
        if self.is_fetching {
            return;
        }

        self.is_fetching = true;
        let tx = self.tx.clone();

        std::thread::spawn(move || match news::fetch() {
            Err(e) => {
                let _ = tx.try_send(ChannelItem::ErrMsg(e.to_string()));
            }
            Ok(v) => {
                let _ = tx.try_send(ChannelItem::NewsItems(v));
            }
        });
    }

    fn popup_message(&mut self, ctx: &Context) {
        let mut is_show = util::timestamp() - self.msg_spec.timestamp < 5_i64;

        let frame = Frame::none()
            .fill(match self.msg_spec.msg_type {
                MsgType::Success => theme::SUCCESS_COLOR,
                MsgType::Warn => theme::WARN_COLOR,
                MsgType::Danger => theme::DANGER_COLOR,
                _ => theme::INFO_COLOR,
            })
            .rounding(0.0)
            .inner_margin(theme::PADDING)
            .stroke(Stroke {
                width: 1.0,
                color: Color32::BLACK,
            });

        Window::new("popup-message")
            .title_bar(false)
            .open(&mut is_show)
            .collapsible(false)
            .auto_sized()
            .constrain(true)
            .interactable(false)
            .fixed_pos(Pos2::new(10.0, 60.0))
            .frame(frame)
            .show(ctx, |ui| {
                ui.label(&self.msg_spec.msg);
            });
    }

    fn show_message(&mut self, msg: String, msg_type: MsgType) {
        self.msg_spec.msg = msg;
        self.msg_spec.msg_type = msg_type;
        self.msg_spec.timestamp = util::timestamp();
    }
}

#[allow(unused)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
