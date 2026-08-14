use alvr_common::ALVR_VERSION;
use alvr_gui_common::theme;
use eframe::egui::{self, Frame, RichText, ScrollArea, Ui};

pub fn about_tab_ui(ui: &mut Ui) {
    ui.label(RichText::new(format!("ALVR 串流端 v{}", *ALVR_VERSION)).size(30.0));
    ui.add_space(10.0);
    ui.hyperlink_to("到 GitHub 上看看我們", "https://github.com/alvr-org/ALVR");
    ui.hyperlink_to("加入我們的 Discord", "https://discord.gg/ALVR");
    ui.hyperlink_to(
        "最新版本",
        "https://github.com/alvr-org/ALVR/releases/latest",
    );
    ui.hyperlink_to(
        "在 Open Collective 上贊助 ALVR",
        "https://opencollective.com/alvr",
    );
    ui.add_space(10.0);
    ui.label("授權條款：");
    Frame::group(ui.style())
        .fill(theme::DARKER_BG)
        .inner_margin(egui::vec2(15.0, 12.0))
        .show(ui, |ui| {
            ScrollArea::new([false, true])
                .id_salt("license_scroll")
                .show(ui, |ui| ui.label(include_str!("../../../../../LICENSE")))
        });
}
