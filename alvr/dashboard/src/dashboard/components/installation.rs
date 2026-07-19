use alvr_gui_common::theme;
use alvr_packets::{FirewallRulesAction, ServerRequest};
use eframe::{
    egui::{Grid, Layout, Ui},
    emath::Align,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

const DRIVER_UPDATE_INTERVAL: Duration = Duration::from_secs(1);

pub enum InstallationTabRequest {
    OpenSetupWizard,
    ServerRequest(ServerRequest),
}

pub struct InstallationTab {
    drivers: Vec<PathBuf>,
    last_update_instant: Instant,
}

impl InstallationTab {
    pub fn new() -> Self {
        Self {
            drivers: vec![],
            last_update_instant: Instant::now(),
        }
    }

    pub fn update_drivers(&mut self, list: Vec<PathBuf>) {
        self.drivers = list;
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Vec<InstallationTabRequest> {
        let mut requests = vec![];

        let now = Instant::now();
        if now > self.last_update_instant + DRIVER_UPDATE_INTERVAL {
            requests.push(InstallationTabRequest::ServerRequest(
                ServerRequest::GetDriverList,
            ));

            self.last_update_instant = now;
        }

        ui.vertical_centered_justified(|ui| {
            theme::card(theme::SECTION_BG).show(ui, |ui| {
                theme::section_title(ui, "設定與防火牆");
                ui.add_space(10.0);

                if ui.button("執行設定精靈").clicked() {
                    requests.push(InstallationTabRequest::OpenSetupWizard);
                }

                ui.add_space(6.0);

                ui.columns(2, |ui| {
                    if ui[0].button("新增防火牆規則").clicked() {
                        requests.push(InstallationTabRequest::ServerRequest(
                            ServerRequest::FirewallRules(FirewallRulesAction::Add),
                        ));
                    }
                    if ui[1].button("移除防火牆規則").clicked() {
                        requests.push(InstallationTabRequest::ServerRequest(
                            ServerRequest::FirewallRules(FirewallRulesAction::Remove),
                        ));
                    }
                });
            });

            ui.add_space(12.0);

            theme::card(theme::SECTION_BG).show(ui, |ui| {
                theme::section_title(ui, "已註冊的驅動程式");
                ui.add_space(10.0);

                Grid::new(0).num_columns(2).show(ui, |ui| {
                    for driver_path in &self.drivers {
                        ui.horizontal(|ui| {
                            ui.add_space(5.0);
                            ui.label(driver_path.to_string_lossy());
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button("移除").clicked() {
                                requests.push(InstallationTabRequest::ServerRequest(
                                    ServerRequest::UnregisterDriver(driver_path.clone()),
                                ));
                            }
                        });
                        ui.end_row();
                    }
                });

                ui.add_space(10.0);

                if ui.button("註冊 ExtraVR 驅動程式").clicked() {
                    requests.push(InstallationTabRequest::ServerRequest(
                        ServerRequest::RegisterAlvrDriver,
                    ));
                }
            });
        });

        requests
    }
}
