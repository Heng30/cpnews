use egui::{
    Color32, ColorImage, Context, FontData, FontDefinitions, FontFamily, Style, Vec2, Visuals,
};
use image;

pub const SPACING: f32 = 4.;
pub const PADDING: f32 = 4.;
pub const ICON_SIZE: Vec2 = Vec2::new(24.0, 24.0);

pub const NEWS_TITLE_FONT_SIZE: f32 = 16.0;
pub const NEWS_TITLE_COLOR: Color32 = Color32::from_rgb(0, 0, 200);

pub const BRAND_COLOR: Color32 = Color32::DARK_BLUE;
pub const LIGHT_COLOR: Color32 = Color32::GRAY;

pub const REFRESH_ICON: &[u8] = include_bytes!("./res/image/refresh.png");
pub const LANG_ICON: &[u8] = include_bytes!("./res/image/lang.png");

pub fn init(ctx: &Context) {
    set_font(ctx);

    ctx.set_visuals(Visuals::light());

    let mut style: Style = (*ctx.style()).clone();
    style.spacing.scroll_bar_width = 2.0;
    style.visuals.hyperlink_color = LIGHT_COLOR;
    ctx.set_style(style);
}

pub fn set_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "SourceHanSerifCN".to_owned(),
        FontData::from_static(include_bytes!("./res/font/SourceHanSerifCN.ttf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "SourceHanSerifCN".to_owned());

    ctx.set_fonts(fonts);
}

pub fn load_image_from_memory(image_data: &[u8]) -> ColorImage {
    let image = image::load_from_memory(image_data).unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}
