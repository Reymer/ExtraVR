use alvr_packets::{FirewallRulesAction, ServerRequest};
use eframe::{
    egui::{Button, Label, Layout, RichText, Ui},
    emath::Align,
};

pub enum SetupWizardRequest {
    ServerRequest(ServerRequest),
    Close { finished: bool },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Welcome = 0,
    ResetSettings = 1,
    HardwareRequirements = 2,
    SoftwareRequirements = 3,
    Firewall = 4,
    Recommendations = 5,
    Finished = 6,
}

fn index_to_page(index: usize) -> Page {
    match index {
        0 => Page::Welcome,
        1 => Page::ResetSettings,
        2 => Page::HardwareRequirements,
        3 => Page::SoftwareRequirements,
        4 => Page::Firewall,
        5 => Page::Recommendations,
        6 => Page::Finished,
        _ => panic!("Invalid page index"),
    }
}

fn page_content(
    ui: &mut Ui,
    subtitle: &str,
    paragraph: &str,
    interactible_content: impl FnMut(&mut Ui),
) {
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.add_space(60.0);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add_space(60.0);
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.add_space(15.0);
                ui.heading(RichText::new(subtitle).size(20.0));
                ui.add(Label::new(RichText::new(paragraph).size(14.0)).wrap());
                ui.add_space(30.0);
                ui.vertical_centered(interactible_content);
            });
        })
    });
}

pub struct SetupWizard {
    page: Page,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            page: Page::Welcome,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<SetupWizardRequest> {
        let mut request = None;

        ui.horizontal(|ui| {
            ui.add_space(60.0);
            ui.vertical(|ui| {
                ui.add_space(30.0);
                ui.heading(RichText::new("歡迎使用 ALVR").size(30.0));
                ui.add_space(5.0);
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(15.0);
                if ui.button("❌").clicked() {
                    request = Some(SetupWizardRequest::Close { finished: false });
                }
            })
        });
        ui.separator();
        match &self.page {
            Page::Welcome => page_content(
                ui,
                "這個設定精靈會協助你設定 ALVR。",
                "",
                |_| (),
            ),
            Page::ResetSettings => page_content(
                ui,
                "重設設定",
                "建議每次更新 ALVR 之後都重設一次設定。",
                |ui| {
                    if ui.button("重設設定").clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::UpdateSession(Box::default()),
                        ));
                    }
                },
            ),
            Page::HardwareRequirements => page_content(
                ui,
                "硬體需求",
                r"ALVR 需要一張獨立且較新的顯示卡。低階的 Intel 內顯可能無法正常運作。
請確認你至少有一個音訊輸出裝置。",
                |_| (),
            ),
            Page::SoftwareRequirements => page_content(
                ui,
                "軟體需求",
                if cfg!(windows) {
                    r"要在 Windows 上串流頭戴裝置的麥克風，需要安裝 Virtual Audio Cable、VB-Cable 或 Voicemeeter"
                } else if cfg!(target_os = "linux") {
                    r"需要 PipeWire（0.3.49 以上版本）音訊系統才能串流音訊與使用麥克風。"
                } else {
                    r"不支援的作業系統"
                },
                #[allow(unused_variables)]
                |ui| {
                    #[cfg(windows)]
                    if ui.button("下載 Virtual Audio Cable（Lite）").clicked() {
                        ui.ctx().open_url(eframe::egui::OpenUrl::same_tab(
                            "https://software.muzychenko.net/freeware/vac470lite.zip",
                        ));
                    }
                },
            ),
            Page::Firewall => page_content(
                ui,
                "防火牆",
                r"為了跟頭戴裝置通訊，需要設定一些防火牆規則。
這需要系統管理員權限！",
                |ui| {
                    if ui.button("新增防火牆規則").clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::FirewallRules(FirewallRulesAction::Add),
                        ));
                    }
                },
            ),
            Page::Recommendations => page_content(
                ui,
                "建議事項",
                r"ALVR 支援多種 PC 硬體與頭戴裝置，但不是所有裝置在預設設定下都能完美運作。如果體驗不夠理想，請嘗試調整解析度、位元率、編碼器等設定。",
                |_| (),
            ),
            Page::Finished => page_content(
                ui,
                "完成",
                r#"你隨時可以從左邊的「安裝」分頁重新啟動這個設定精靈。"#,
                |_| (),
            ),
        };

        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            ui.add_space(30.0);
            ui.horizontal(|ui| {
                ui.add_space(15.0);
                if self.page == Page::Finished {
                    if ui.button("完成").clicked() {
                        request = Some(SetupWizardRequest::Close { finished: true });
                    }
                } else if ui.button("下一步").clicked() {
                    self.page = index_to_page(self.page as usize + 1);
                }
                if ui
                    .add_visible(self.page != Page::Welcome, Button::new("上一步"))
                    .clicked()
                {
                    self.page = index_to_page(self.page as usize - 1);
                }
            });
            ui.separator();
        });

        request
    }
}
