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
const NO_NOTIFICATIONS_MESSAGE: &str = "沒有新通知";
const NOTIFICATION_TIPS: &[&str] = &[
    // The following tips are ordered roughtly in the order settings appear
    r#"如果改了某些設定後開始出現崩潰，可以到「Installation」分頁重新執行「Run setup wizard」，並按下「Reset settings」來重設 ALVR。"#,
    r#"有些設定預設是隱藏的。點旁邊的「Expand」按鈕可以展開子選單。"#,
    r#"強烈建議 ALVR 裡的音訊設定維持預設值，改在工作列的系統匣裡調整預設音訊裝置。"#,
    r#"調高「Maximum buffering」可能會減少卡頓，但代價是延遲會變高。"#,
    r#"某些顯示卡有時需要在 h264 與 HEVC 編碼間切換，才能避免崩潰或退回軟體編碼。"#,
    r#"如果你用 NVIDIA 顯示卡，建議用高位元率的 H264；如果是 AMD 顯示卡，HEVC 畫質可能比較好。"#,
    r#"如果畫面出現「雪花」閃爍，把解析度降到「Low」並停用「Foveated encoding」。"#,
    r#"調高「Color correction」->「Sharpness」可能會改善主觀畫質。"#,
    r#"如果外接控制器或追蹤器跟 ALVR 追蹤空間對不準，試著在「Extra openvr props」加一個項目，再設定自訂的「Tracking system name」。"#,
    r#"要改變控制器的顯示外觀，設定「Headset」->「Controllers」->「Emulation mode」。"#,
    r#"ALVR 支援自訂按鍵綁定！有需要的話歡迎到 Discord 伺服器問我們。"#,
    r#"ALVR 支援手勢追蹤（「Presets」->「Hand tracking interaction」->「ALVR Bindings」）。詳細用法請參考 wiki：https://github.com/alvr-org/ALVR/wiki/Hand-tracking-controller-bindings。"#,
    r#"如果手勢追蹤造成困擾，可以在「Controllers」->「Gestures」停用它，或者啟用「Gestures」->「Only touch」。"#,
    r#"可以用「Controllers」->「Prediction」微調控制器的反應靈敏度。"#,
    r#"如果畫面上的控制器/手部模型跟實體控制器對不上，可以在「Headset」->「Controllers」->「Left controller position/rotation offset」調整偏移（會同時影響兩支控制器）。"#,
    r#"使用外接追蹤器或控制器時，「Position/Rotation recentering mode」都應該設成「Disabled」。"#,
    r#"可以啟用傾斜模式：把「Position recentering mode」設成「Local」、「Rotation recentering mode」設成「Tilted」。"#,
    r#"如果常遇到畫面錯位，可以用「Avoid video glitching」，用卡頓換取畫面正確。"#,
    r#"可以用「On connect/disconnect script」，在頭戴裝置連線/斷線時執行自訂指令或程式。"#,
    r#"如果要回報問題並附上 log 檔，啟用「Extra」->「Logging」->「Log to disk」即可，log 會存在「session_log.txt」裡。"#,
    r#"為了除錯用途，可以啟用「Log tracking」、「Log button presses」與「Log haptics」，並透過 ws://localhost:8082/api/events 這個 websocket 取得資料"#,
    r#"如果要回報問題並分享 log，建議啟用「Extra」->「Logging」->「Prefer backtrace」。"#,
    r#"切換「Show notification tip」可以快速循環顯示像這樣的小提示"#,
    r#"如果要跟 ALVR 一起使用身體追蹤器或其他 SteamVR 驅動程式，把「Driver launch action」設成「Unregister ALVR at shutdown」"#,
    r#"啟用「Extra」->「Steamvr Launcher」->「Open and close SteamVR with dashboard」會方便不少"#,
    r#"如果要分享錄影來回報問題，可以啟用「Extra」->「Capture」->「Rolling video files」來限制上傳檔案大小"#,
    // Miscellaneous
    r#"如果裝置列表裡沒看到你的頭戴裝置，可能是它在不同的子網路。試著用裝置裡顯示的 IP，透過「Add device manually」手動加入。"#,
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
