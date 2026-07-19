mod components;

use self::components::{
    DevicesTab, LogsTab, NotificationBar, SettingsTab, SetupWizard, SetupWizardRequest,
};
use crate::{
    dashboard::components::{CloseAction, NewVersionPopup, StatisticsTab},
    DataSources,
};
use alvr_common::parking_lot::{Condvar, Mutex};
use alvr_events::EventType;
use alvr_gui_common::theme;
use alvr_packets::{PathValuePair, ServerRequest};
use alvr_session::SessionConfig;
use eframe::egui::{self, Align, CentralPanel, Frame, Layout, Margin, RichText, Stroke};
use std::{collections::BTreeMap, sync::Arc};

// ExtraVR ships the installer, which handles firewall rules and SteamVR driver
// registration, so `Installation` and `Debug` are deliberately left out of the
// sidebar list built in `Dashboard::new`. Their tabs still render correctly, so
// the variants are kept for whenever they need to be listed again.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tab {
    Devices,
    Statistics,
    Settings,
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    Installation,
    Logs,
    #[allow(dead_code)]
    Debug,
    About,
}

#[derive(Clone, Copy)]
enum TabIcon {
    Devices,
    Statistics,
    Settings,
    #[allow(dead_code)]
    Installation,
    Logs,
    #[allow(dead_code)]
    Debug,
    About,
}

/// Draw a clean monochrome line icon centred in `rect` — replaces the mismatched
/// colour emoji that egui renders for the sidebar tabs.
fn draw_tab_icon(painter: &egui::Painter, rect: egui::Rect, icon: TabIcon, color: egui::Color32) {
    let c = rect.center();
    let s = Stroke::new(1.8_f32, color);
    let p = |dx: f32, dy: f32| c + egui::vec2(dx, dy);

    match icon {
        TabIcon::Devices => {
            painter.circle_stroke(p(-6.0, 0.0), 2.4, s);
            painter.circle_stroke(p(6.0, 0.0), 2.4, s);
            painter.line_segment([p(-3.6, 0.0), p(3.6, 0.0)], s);
        }
        TabIcon::Statistics => {
            painter.line_segment([p(-8.0, -7.0), p(-8.0, 7.0)], s);
            painter.line_segment([p(-8.0, 7.0), p(8.0, 7.0)], s);
            painter.add(egui::Shape::line(
                vec![p(-6.5, 4.0), p(-2.0, -1.0), p(2.0, 1.5), p(7.0, -5.0)],
                s,
            ));
        }
        TabIcon::Settings => {
            for (y, kx) in [(-5.0_f32, -2.0_f32), (0.0, 3.5), (5.0, -4.0)] {
                painter.line_segment([p(-8.0, y), p(8.0, y)], s);
                painter.circle_filled(p(kx, y), 2.3, color);
            }
        }
        TabIcon::Installation => {
            painter.line_segment([p(0.0, -7.0), p(0.0, 3.0)], s);
            painter.line_segment([p(-3.5, -0.5), p(0.0, 3.0)], s);
            painter.line_segment([p(3.5, -0.5), p(0.0, 3.0)], s);
            painter.line_segment([p(-6.0, 6.5), p(6.0, 6.5)], s);
        }
        TabIcon::Logs => {
            painter.line_segment([p(-7.0, -4.5), p(7.0, -4.5)], s);
            painter.line_segment([p(-7.0, 0.0), p(7.0, 0.0)], s);
            painter.line_segment([p(-7.0, 4.5), p(2.0, 4.5)], s);
        }
        TabIcon::Debug => {
            painter.add(egui::Shape::line(
                vec![p(-3.0, -5.5), p(-8.0, 0.0), p(-3.0, 5.5)],
                s,
            ));
            painter.add(egui::Shape::line(
                vec![p(3.0, -5.5), p(8.0, 0.0), p(3.0, 5.5)],
                s,
            ));
            painter.line_segment([p(1.8, -6.0), p(-1.8, 6.0)], s);
        }
        TabIcon::About => {
            painter.circle_stroke(c, 8.0, s);
            painter.circle_filled(p(0.0, -3.6), 1.3, color);
            painter.line_segment([p(0.0, -0.5), p(0.0, 4.6)], s);
        }
    }
}

pub struct Dashboard {
    data_sources: DataSources,
    just_opened: bool,
    server_restarting: Arc<Mutex<bool>>,
    server_restarting_condvar: Arc<Condvar>,
    selected_tab: Tab,
    tab_labels: BTreeMap<Tab, (TabIcon, &'static str)>,
    connections_tab: DevicesTab,
    statistics_tab: StatisticsTab,
    settings_tab: SettingsTab,
    #[cfg(not(target_arch = "wasm32"))]
    installation_tab: components::InstallationTab,
    logs_tab: LogsTab,
    notification_bar: NotificationBar,
    setup_wizard: SetupWizard,
    new_version_popup: Option<components::NewVersionPopup>,
    setup_wizard_open: bool,
    session: Option<SessionConfig>,
}

impl Dashboard {
    pub fn new(creation_context: &eframe::CreationContext<'_>, data_sources: DataSources) -> Self {
        alvr_gui_common::theme::set_theme(&creation_context.egui_ctx);

        data_sources.request(ServerRequest::GetSession);

        Self {
            data_sources,
            just_opened: true,
            server_restarting: Arc::new(Mutex::new(false)),
            server_restarting_condvar: Arc::new(Condvar::new()),
            selected_tab: Tab::Devices,
            tab_labels: [
                (Tab::Devices, (TabIcon::Devices, "裝置")),
                (Tab::Statistics, (TabIcon::Statistics, "統計")),
                (Tab::Settings, (TabIcon::Settings, "設定")),
                (Tab::Logs, (TabIcon::Logs, "日誌")),
                (Tab::About, (TabIcon::About, "關於")),
            ]
            .into_iter()
            .collect(),
            connections_tab: DevicesTab::new(),
            statistics_tab: StatisticsTab::new(),
            settings_tab: SettingsTab::new(),
            #[cfg(not(target_arch = "wasm32"))]
            installation_tab: components::InstallationTab::new(),
            logs_tab: LogsTab::new(),
            notification_bar: NotificationBar::new(),
            setup_wizard: SetupWizard::new(),
            setup_wizard_open: false,
            session: None,
            new_version_popup: None,
        }
    }

    // This call may block
    fn restart_steamvr(&self, requests: &mut Vec<ServerRequest>) {
        requests.push(ServerRequest::RestartSteamvr);

        let mut server_restarting_lock = self.server_restarting.lock();

        if *server_restarting_lock {
            self.server_restarting_condvar
                .wait(&mut server_restarting_lock);
        }

        *server_restarting_lock = true;

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn({
            let server_restarting = Arc::clone(&self.server_restarting);
            let condvar = Arc::clone(&self.server_restarting_condvar);
            move || {
                crate::steamvr_launcher::LAUNCHER.lock().restart_steamvr();

                *server_restarting.lock() = false;
                condvar.notify_one();
            }
        });
    }
}

impl eframe::App for Dashboard {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        let mut requests = vec![];

        let connected_to_server = self.data_sources.server_connected();

        while let Some(event) = self.data_sources.poll_event() {
            self.logs_tab.push_event(event.inner.clone());

            match event.inner.event_type {
                EventType::Log(log_event) => {
                    self.notification_bar
                        .push_notification(log_event, event.from_dashboard);
                }
                EventType::GraphStatistics(graph_statistics) => self
                    .statistics_tab
                    .update_graph_statistics(graph_statistics),
                EventType::StatisticsSummary(statistics) => {
                    self.statistics_tab.update_statistics(statistics)
                }
                EventType::Session(session) => {
                    let settings = session.to_settings();

                    self.connections_tab.update_client_list(&session);
                    self.settings_tab.update_session(&session.session_settings);
                    self.logs_tab.update_settings(&settings);
                    self.notification_bar.update_settings(&settings);
                    if self.just_opened {
                        self.just_opened = false;
                    }

                    self.session = Some(*session);
                }
                EventType::ServerRequestsSelfRestart => self.restart_steamvr(&mut requests),
                #[cfg(not(target_arch = "wasm32"))]
                EventType::DriversList(list) => self.installation_tab.update_drivers(list),
                EventType::Adb(adb_event) => self
                    .connections_tab
                    .update_adb_download_progress(adb_event.download_progress),
                EventType::NewVersionFound { version, message } => {
                    self.new_version_popup = Some(NewVersionPopup::new(version, message));
                }
                EventType::DebugGroup { .. }
                | EventType::Tracking(_)
                | EventType::Buttons(_)
                | EventType::Haptics(_) => (),
            }
        }

        if *self.server_restarting.lock() {
            CentralPanel::default().show(context, |ui| {
                // todo: find a way to center both vertically and horizontally
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading(RichText::new("SteamVR 正在重新啟動").size(30.0));
                });
            });

            return;
        }

        if self.setup_wizard_open {
            CentralPanel::default().show(context, |ui| {
                if let Some(request) = self.setup_wizard.ui(ui) {
                    match request {
                        SetupWizardRequest::ServerRequest(request) => {
                            requests.push(request);
                        }
                        SetupWizardRequest::Close { finished } => {
                            if finished {
                                requests.push(ServerRequest::SetValues(vec![PathValuePair {
                                    path: alvr_packets::parse_path(
                                        "session_settings.extra.open_setup_wizard",
                                    ),
                                    value: serde_json::Value::Bool(false),
                                }]))
                            }

                            self.setup_wizard_open = false;
                        }
                    }
                }
            });
        } else {
            egui::TopBottomPanel::top("nav_bar")
                .frame(
                    Frame::new()
                        .fill(theme::LIGHTER_BG)
                        .inner_margin(Margin::symmetric(14, 9))
                        .stroke(Stroke::new(1.0_f32, theme::SEPARATOR_BG)),
                )
                .show(context, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        ui.heading(
                            RichText::new("ExtraVR")
                                .size(20.0)
                                .strong()
                                .color(theme::ACCENT),
                        );
                        egui::warn_if_debug_build(ui);
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);

                        for (tab, (icon, name)) in &self.tab_labels {
                            let selected = self.selected_tab == *tab;
                            let galley = ui.painter().layout_no_wrap(
                                (*name).to_owned(),
                                egui::FontId::proportional(15.0),
                                theme::FG,
                            );
                            let (rect, resp) = ui.allocate_at_least(
                                egui::vec2(galley.size().x + 46.0, 30.0),
                                egui::Sense::click(),
                            );
                            if selected {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(8),
                                    theme::ACCENT,
                                );
                            } else if resp.hovered() {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(8),
                                    theme::SECTION_BG,
                                );
                            }
                            let fg = if selected {
                                theme::ON_ACCENT
                            } else {
                                theme::FG
                            };
                            draw_tab_icon(
                                ui.painter(),
                                egui::Rect::from_center_size(
                                    egui::pos2(rect.left() + 19.0, rect.center().y),
                                    egui::vec2(18.0, 18.0),
                                ),
                                *icon,
                                fg,
                            );
                            ui.painter().text(
                                egui::pos2(rect.left() + 34.0, rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                name,
                                egui::FontId::proportional(15.0),
                                fg,
                            );
                            if resp.clicked() {
                                self.selected_tab = *tab;
                            }
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if connected_to_server {
                                if ui.button("重新啟動 SteamVR").clicked() {
                                    self.restart_steamvr(&mut requests);
                                }
                            } else if ui.button("啟動 SteamVR").clicked() {
                                crate::steamvr_launcher::LAUNCHER.lock().launch_steamvr();
                            }
                            ui.add_space(8.0);
                            let (dot, status) = if connected_to_server {
                                (theme::OK_GREEN, "SteamVR：已連線")
                            } else {
                                (theme::KO_RED, "SteamVR：已中斷連線")
                            };
                            ui.label(RichText::new("●").color(dot).size(13.0))
                                .on_hover_text(status);
                        });
                    });
                });

            CentralPanel::default()
                .frame(Frame::new().inner_margin(Margin::same(20)).fill(theme::BG))
                .show(context, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        match self.selected_tab {
                            Tab::Devices => {
                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        requests.extend(
                                            self.connections_tab.ui(ui, connected_to_server),
                                        );
                                    });
                            }
                            Tab::Statistics => {
                                if let Some(request) = self.statistics_tab.ui(ui) {
                                    requests.push(request);
                                }
                            }
                            Tab::Settings => {
                                requests.extend(self.settings_tab.ui(ui));
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            Tab::Installation => {
                                for request in self.installation_tab.ui(ui) {
                                    match request {
                                        components::InstallationTabRequest::OpenSetupWizard => {
                                            self.setup_wizard_open = true
                                        }
                                        components::InstallationTabRequest::ServerRequest(
                                            request,
                                        ) => {
                                            requests.push(request);
                                        }
                                    }
                                }
                            }
                            Tab::Logs => self.logs_tab.ui(ui),
                            Tab::Debug => {
                                if let Some(request) = components::debug_tab_ui(ui) {
                                    requests.push(request);
                                }
                            }
                            Tab::About => components::about_tab_ui(ui),
                        }
                    })
                });
        }

        let shutdown_alvr = || {
            self.data_sources.request(ServerRequest::ShutdownSteamvr);

            crate::steamvr_launcher::LAUNCHER
                .lock()
                .ensure_steamvr_shutdown();
        };

        if let Some(popup) = &self.new_version_popup {
            if let Some(action) = popup.ui(context, shutdown_alvr) {
                if let CloseAction::CloseWithRequest(request) = action {
                    requests.push(request);
                }

                self.new_version_popup = None;
            }
        }

        for request in requests {
            self.data_sources.request(request);
        }

        if context.input(|state| state.viewport().close_requested())
            && self.session.as_ref().is_some_and(|s| {
                s.to_settings()
                    .extra
                    .steamvr_launcher
                    .open_close_steamvr_with_dashboard
            })
        {
            shutdown_alvr();
        }
    }
}
