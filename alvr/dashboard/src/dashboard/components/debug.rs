use alvr_packets::ServerRequest;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    ui.label(
        "用下方按鈕透過 ALVR 錄製並不適合拿來擷取遊戲畫面。
如需擷取遊戲畫面，請改用其他錄製方式，例如透過頭戴裝置或桌面 VR 輸出。",
    );

    ui.columns(4, |ui| {
        if ui[0].button("擷取畫格").clicked() {
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

    request
}
