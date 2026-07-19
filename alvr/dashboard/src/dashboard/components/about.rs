use alvr_gui_common::theme;
use eframe::egui::{self, RichText, ScrollArea, Ui};

pub fn about_tab_ui(ui: &mut Ui) {
    theme::card(theme::SECTION_BG)
        .inner_margin(egui::vec2(18.0, 16.0))
        .show(ui, |ui| {
            ui.label(
                RichText::new("ExtraVR 串流端 v1.0.0")
                    .size(30.0)
                    .strong()
                    .color(theme::ACCENT),
            );
        });

    ui.add_space(12.0);

    theme::card(theme::SECTION_BG).show(ui, |ui| {
        theme::section_title(ui, "授權條款");
        ui.add_space(10.0);
        theme::card(theme::DARKER_BG)
            .inner_margin(egui::vec2(15.0, 12.0))
            .show(ui, |ui| {
                ScrollArea::new([false, true])
                    .id_salt("license_scroll")
                    .show(ui, |ui| ui.label(include_str!("../../../../../LICENSE")))
            });
    });
}
