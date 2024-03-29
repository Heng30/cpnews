use super::{
    app::{App, CurrentPanel},
    theme,
    tr::tr,
    version,
};
use egui::{Button, FontId, RichText, Ui};

#[derive(Default, Debug, Clone)]
pub struct About;

pub fn ui(app: &mut App, ui: &mut Ui) {
    if ui
        .add(
            Button::image_and_text(
                app.back_icon.clone().unwrap().id(),
                theme::BACK_ICON_SIZE,
                RichText::new(tr(app.conf.ui.is_cn, "关于"))
                    .font(FontId::proportional(theme::NEWS_TITLE_FONT_SIZE)),
            )
            .frame(false),
        )
        .clicked()
    {
        app.current_panel = CurrentPanel::News;
    }

    ui.vertical_centered(|ui| {
        let title = format!("{} {}", tr(app.conf.ui.is_cn, "加密新闻"),  version::VERSION);
        let address = "0xf1199999751b1a3A74590adBf95401D19AB30014";
        let etherscan = "https://etherscan.io/address/";

        ui.add_space(theme::SPACING * 4.);
        ui.heading(title);
        ui.add_space(theme::SPACING);

        if app.conf.ui.is_cn {
            ui.label("基于egui。版权2022-2030 Heng30公司有限公司，保留所有权利。该程序按原样提供，不提供任何形式的保证，包括设计，适销性和特定用途的保证。");
        } else {
            ui.label("Based on egui. Copyright 2022-2030 The Heng30 Company Ltd. All rights reserved. The program is provided AS IS with NO WARRANTY OF ANY KIND, INCLUDING THE WARRANTY OF DESIGN, MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.");
        }

        ui.add_space(theme::SPACING * 2.);
        if app.conf.ui.is_cn {
            ui.label("🎉❤给我买一杯咖啡(MetaMask)❤🎉");
        } else {
            ui.label("🎉❤Buy Me a Coffee(MetaMask)❤🎉");
        }

        ui.add_space(theme::SPACING);

        if ui.link(address).clicked() {
            if let Err(e) = webbrowser::open(&format!("{etherscan}{address}")) {
                log::warn!("{e:?}");
            }
        }
    });
}
