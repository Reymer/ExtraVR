use alvr_common::{LogEntry, LogSeverity};
use alvr_gui_common::theme::{self, log_colors};
use alvr_session::Settings;
use eframe::{
    egui::{self, Frame, Label, Layout, RichText, TopBottomPanel},
    emath::Align,
    epaint::{Color32, Stroke},
};
use rand::seq::IndexedRandom;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use instant::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(5);
const NO_NOTIFICATIONS_MESSAGE: &str = "沒有新的通知";
const NOTIFICATION_TIPS: &[&str] = &[
    // The following tips are ordered roughtly in the order settings appear
    r#"如果您在變更某些設定後開始發生當機，請從「Installation」分頁重新執行「Run setup wizard」，並點擊「Reset settings」來重設 ExtraVR。"#,
    r#"部分設定預設為隱藏。點擊某些設定旁的「Expand」按鈕即可展開子選單。"#,
    r#"強烈建議在 ExtraVR 中將音訊設定保持為預設值，並於工作列系統匣中修改預設音訊裝置。"#,
    r#"提高「Maximum buffering」可能減少畫面卡頓，但會增加延遲。"#,
    r#"在某些 GPU 上，有時需要在 h264 與 HEVC 編解碼器之間切換，以修正當機或改用軟體編碼。"#,
    r#"如果您使用 NVIDIA GPU，最好使用高位元率的 H264；如果您使用 AMD GPU，HEVC 可能看起來更好。"#,
    r#"如果您遇到「白色雪花」閃爍，請將解析度降低為「Low」並停用「Foveated encoding」。"#,
    r#"提高「Color correction」->「Sharpness」可能改善觀感上的畫質。"#,
    r#"如果您在將外部控制器或追蹤器同步到 ExtraVR 追蹤空間時遇到問題，請在「Extra openvr props」中新增一個元素，然後設定自訂的「Tracking system name」。"#,
    r#"若要變更控制器的外觀，請設定「Headset」->「Controllers」->「Emulation mode」。"#,
    r#"ExtraVR 支援自訂按鍵綁定！如果需要協助，請到我們的 Discord 伺服器詢問。"#,
    r#"ExtraVR 支援手部追蹤手勢（「Presets」->「Hand tracking interaction」->「ALVR Bindings」）。請參閱 wiki 了解如何正確使用：https://github.com/alvr-org/ALVR/wiki/Hand-tracking-controller-bindings。"#,
    r#"如果手部追蹤手勢造成困擾，您可以在「Controllers」->「Gestures」中停用它們。或者，您也可以啟用「Gestures」->「Only touch」。"#,
    r#"您可以透過「Controllers」->「Prediction」微調控制器的反應靈敏度。"#,
    r#"如果畫面中的控制器/手部模型與實體控制器不符，您可以在「Headset」->「Controllers」->「Left controller position/rotation offset」中調整偏移量（會同時影響兩個控制器）。"#,
    r#"使用外部追蹤器或控制器時，您應將「Position/Rotation recentering mode」皆設為「Disabled」。"#,
    r#"您可以啟用傾斜模式。將「Position recentering mode」設為「Local」，並將「Rotation recentering mode」設為「Tilted」。"#,
    r#"如果您經常遇到影像破圖，可以使用「Avoid video glitching」以卡頓畫面來換取改善。"#,
    r#"您可以使用「On connect/disconnect script」，在頭戴裝置連線/中斷連線時執行自訂的指令/程式。"#,
    r#"如果您想回報錯誤並取得日誌檔案，請啟用「Extra」->「Logging」->「Log to disk」。日誌會存放在「session_log.txt」中。"#,
    r#"為了進行技術研究，您可以啟用「Log tracking」、「Log button presses」和「Log haptics」。您可以透過 ws://localhost:8082/api/events 的 websocket 取得資料"#,
    r#"如果您想回報錯誤並分享日誌，您應啟用「Extra」->「Logging」->「Prefer backtrace」。"#,
    r#"您可以透過切換「Show notification tip」快速瀏覽像這樣的提示"#,
    r#"如果您想同時將身體追蹤器或其他 SteamVR 驅動程式與 ALVR 一起使用，請將「Driver launch action」設為「Unregister ALVR at shutdown」"#,
    r#"啟用「Extra」->「Steamvr Launcher」->「Open and close SteamVR with dashboard」會很方便"#,
    r#"如果您想分享影片錄製以回報錯誤，您可以啟用「Extra」->「Capture」->「Rolling video files」以限制上傳的檔案大小"#,
    // Miscellaneous
    r#"如果您的頭戴裝置未出現在裝置清單中，它可能位於不同的子網路。請嘗試使用「Add device manually」，並輸入裝置內顯示的 IP。"#,
];

pub struct NotificationBar {
    message: String,
    current_level: LogSeverity,
    receive_instant: Instant,
    min_notification_level: LogSeverity,
    tip_message: Option<String>,
    expanded: bool,
}

impl NotificationBar {
    pub fn new() -> Self {
        Self {
            message: NO_NOTIFICATIONS_MESSAGE.into(),
            current_level: LogSeverity::Debug,
            receive_instant: Instant::now(),
            min_notification_level: LogSeverity::Debug,
            tip_message: None,
            expanded: false,
        }
    }

    pub fn update_settings(&mut self, settings: &Settings) {
        self.min_notification_level = settings.extra.logging.notification_level;

        if settings.extra.logging.show_notification_tip {
            if self.tip_message.is_none() {
                self.tip_message = NOTIFICATION_TIPS
                    .choose(&mut rand::rng())
                    .map(|s| format!("提示：{s}"));
            }
        } else {
            self.tip_message = None;
        }
    }

    pub fn push_notification(&mut self, event: LogEntry, from_dashboard: bool) {
        let now = Instant::now();
        let min_severity = if from_dashboard {
            if cfg!(debug_assertions) {
                LogSeverity::Debug
            } else {
                LogSeverity::Info
            }
        } else {
            self.min_notification_level
        };

        if event.severity >= min_severity
            && (now > self.receive_instant + TIMEOUT || event.severity >= self.current_level)
        {
            self.message = event.content;
            self.current_level = event.severity;
            self.receive_instant = now;
        }
    }

    pub fn ui(&mut self, context: &egui::Context) {
        let now = Instant::now();
        if now > self.receive_instant + TIMEOUT {
            self.message = self
                .tip_message
                .clone()
                .unwrap_or_else(|| NO_NOTIFICATIONS_MESSAGE.into());
            self.current_level = LogSeverity::Debug;
        }

        let (fg, bg) = match self.current_level {
            LogSeverity::Error => (Color32::BLACK, log_colors::ERROR_LIGHT),
            LogSeverity::Warning => (Color32::BLACK, log_colors::WARNING_LIGHT),
            LogSeverity::Info => (Color32::BLACK, log_colors::INFO_LIGHT),
            LogSeverity::Debug => (theme::FG, theme::LIGHTER_BG),
        };

        let mut bottom_bar = TopBottomPanel::bottom("bottom_panel").frame(
            Frame::default()
                .inner_margin(egui::vec2(10.0, 5.0))
                .fill(bg)
                .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
        );
        let alignment = if !self.expanded {
            bottom_bar = bottom_bar.max_height(26.0);

            Align::TOP
        } else {
            Align::Center
        };

        bottom_bar.show(context, |ui| {
            ui.with_layout(Layout::right_to_left(alignment), |ui| {
                if !self.expanded {
                    if ui.small_button("展開").clicked() {
                        self.expanded = true;
                    }
                } else if ui.button("收合").clicked() {
                    self.expanded = false;
                }
                ui.with_layout(Layout::left_to_right(alignment), |ui| {
                    ui.add(Label::new(RichText::new(&self.message).color(fg).size(12.0)).wrap());
                })
            })
        });
    }
}
