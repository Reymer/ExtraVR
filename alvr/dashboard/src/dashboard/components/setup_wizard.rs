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
                ui.heading(RichText::new("歡迎使用 ExtraVR").size(30.0));
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
                "本設定精靈將協助您設定 ExtraVR。",
                "",
                |_| (),
            ),
            Page::ResetSettings => page_content(
                ui,
                "重設設定",
                "建議您每次更新 ExtraVR 後都重設設定。",
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
                r"ExtraVR 需要獨立且較新的顯示卡。低階的 Intel 內建顯示卡可能無法運作。
請確保您至少有一個輸出音訊裝置。",
                |_| (),
            ),
            Page::SoftwareRequirements => page_content(
                ui,
                "軟體需求",
                if cfg!(windows) {
                    r"在 Windows 上要串流頭戴裝置的麥克風，您需要安裝 Virtual Audio Cable、VB-Cable 或 Voicemeeter"
                } else if cfg!(target_os = "linux") {
                    r"您需要 PipeWire（0.3.49 以上版本）音訊系統，才能串流音訊並使用麥克風。"
                } else {
                    r"不支援的作業系統"
                },
                #[allow(unused_variables)]
                |ui| {
                    #[cfg(windows)]
                    if ui.button("下載 Virtual Audio Cable (Lite)").clicked() {
                        ui.ctx().open_url(eframe::egui::OpenUrl::same_tab(
                            "https://software.muzychenko.net/freeware/vac470lite.zip",
                        ));
                    }
                },
            ),
            Page::Firewall => page_content(
                ui,
                "防火牆",
                r"為了與頭戴裝置通訊，需要設定一些防火牆規則。
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
                r"ExtraVR 支援多種類型的電腦硬體與頭戴裝置，但並非全部都能以預設值正常運作。如果您的 ExtraVR 體驗不理想，請嘗試調整解析度、位元率、編碼器等不同設定。",
                |_| (),
            ),
            Page::Finished => page_content(
                ui,
                "完成",
                r#"您隨時可以從左側的「安裝」分頁重新啟動此設定精靈。"#,
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
