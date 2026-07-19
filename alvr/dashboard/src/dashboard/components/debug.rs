use alvr_gui_common::theme;
use alvr_packets::ServerRequest;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    theme::card(theme::SECTION_BG).show(ui, |ui| {
        theme::section_title(ui, "錄製與擷取");
        ui.add_space(10.0);

        ui.label(
            "使用下方按鈕從 ALVR 錄製並不適合用來擷取遊戲畫面。
若要擷取遊戲畫面，請使用其他錄製方式，例如透過頭戴裝置或桌面 VR 輸出。",
        );

        ui.add_space(10.0);

        ui.columns(4, |ui| {
            if ui[0].button("擷取畫面").clicked() {
                request = Some(ServerRequest::CaptureFrame);
            }

            if ui[1].button("插入 IDR").clicked() {
                request = Some(ServerRequest::InsertIdr);
            }

            if ui[2].button("開始錄製").clicked() {
                request = Some(ServerRequest::StartRecording);
            }

            if ui[3].button("停止錄製").clicked() {
                request = Some(ServerRequest::StopRecording);
            }
        });
    });

    request
}
