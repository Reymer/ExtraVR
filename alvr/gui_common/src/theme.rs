use egui::{self, Color32, Context, CornerRadius, Stroke, TextStyle, ThemePreference, Visuals};

// Tech / minimal theme: cool near-black surfaces + a single electric-cyan accent.
pub const ACCENT: Color32 = Color32::from_rgb(0, 224, 255); // electric cyan
pub const BG: Color32 = Color32::from_rgb(15, 19, 26); // main surface (#0F131A)
pub const LIGHTER_BG: Color32 = Color32::from_rgb(22, 28, 38); // cards / panels (#161C26)
pub const SECTION_BG: Color32 = Color32::from_rgb(22, 28, 38); // sections (#161C26)
pub const DARKER_BG: Color32 = Color32::from_rgb(11, 14, 19); // deepest wells (#0B0E13)
pub const SEPARATOR_BG: Color32 = Color32::from_rgb(40, 50, 64); // hairlines (#283240)
pub const FG: Color32 = Color32::from_rgb(232, 237, 242); // crisp cool white (#E8EDF2)

// Text/icons drawn on top of the accent fill (needs to stay readable on cyan).
pub const ON_ACCENT: Color32 = Color32::from_rgb(6, 15, 22); // near-black ink (#060F16)

pub const OK_GREEN: Color32 = Color32::from_rgb(45, 217, 166); // mint (#2DD9A6)
pub const KO_RED: Color32 = Color32::from_rgb(255, 77, 109); // clean coral-red (#FF4D6D)

pub mod log_colors {
    use egui::epaint::Color32;

    pub const ERROR_LIGHT: Color32 = Color32::from_rgb(255, 50, 50);
    pub const WARNING_LIGHT: Color32 = Color32::from_rgb(205, 147, 9);
    pub const INFO_LIGHT: Color32 = Color32::from_rgb(134, 171, 241);
    pub const DEBUG_LIGHT: Color32 = Color32::LIGHT_GRAY;
    pub const EVENT_LIGHT: Color32 = Color32::GRAY;
}

// Graph colors
pub mod graph_colors {
    use egui::Color32;

    // Colors taken from https://colorhunt.co/palette/ff6b6bffd93d6bcb774d96ff
    pub const RENDER_EXTERNAL: Color32 = Color32::from_rgb(64, 64, 64);
    pub const RENDER_EXTERNAL_LABEL: Color32 = Color32::GRAY;
    pub const RENDER: Color32 = Color32::RED;
    pub const IDLE: Color32 = Color32::from_rgb(255, 217, 61);
    pub const TRANSCODE: Color32 = Color32::from_rgb(107, 203, 119);
    pub const NETWORK: Color32 = Color32::from_rgb(77, 150, 255);

    pub const SERVER_FPS: Color32 = Color32::LIGHT_BLUE;
    pub const CLIENT_FPS: Color32 = Color32::KHAKI;

    pub const INITIAL_CALCULATED_THROUGHPUT: Color32 = Color32::GRAY;
    pub const ENCODER_DECODER_LATENCY_LIMITER: Color32 = TRANSCODE;
    pub const NETWORK_LATENCY_LIMITER: Color32 = NETWORK;
    pub const MIN_MAX_LATENCY_THROUGHPUT: Color32 = Color32::RED;
    pub const REQUESTED_BITRATE: Color32 = Color32::GREEN;
    pub const RECORDED_THROUGHPUT: Color32 = Color32::KHAKI;
    pub const RECORDED_BITRATE: Color32 = super::FG;
}

pub fn set_theme(ctx: &Context) {
    install_cjk_fonts(ctx);

    ctx.set_theme(ThemePreference::Dark);

    let mut style = (*ctx.style()).clone();
    style.spacing.slider_width = 200_f32; // slider width can only be set globally
    style.spacing.item_spacing = egui::vec2(15.0, 15.0);
    style.spacing.button_padding = egui::vec2(10.0, 10.0);
    style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 14.0;

    ctx.set_style(style);

    let mut visuals = Visuals::dark();

    // Rounded pill-style corners on all interactive widgets (chips, dropdowns, buttons).
    let corner_radius = CornerRadius::same(8);

    // Idle vs hovered widget surfaces: cool slate panels that lift slightly on hover.
    let panel = Color32::from_rgb(27, 36, 48); // #1B2430
    let panel_hover = Color32::from_rgb(36, 50, 68); // #243244

    // Global surfaces
    visuals.panel_fill = BG;
    visuals.window_fill = BG;
    visuals.extreme_bg_color = DARKER_BG; // text fields / deep wells
    visuals.faint_bg_color = DARKER_BG; // zebra striping
    visuals.hyperlink_color = ACCENT;

    // Non-interactive: labels, frames, separators
    visuals.widgets.noninteractive.bg_fill = BG;
    visuals.widgets.noninteractive.weak_bg_fill = BG;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, SEPARATOR_BG);
    visuals.widgets.noninteractive.corner_radius = corner_radius;

    // Inactive: idle buttons / combo boxes
    visuals.widgets.inactive.bg_fill = panel;
    visuals.widgets.inactive.weak_bg_fill = panel;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, SEPARATOR_BG);
    visuals.widgets.inactive.corner_radius = corner_radius;

    // Hovered: cyan outline is the signature "tech" highlight
    visuals.widgets.hovered.bg_fill = panel_hover;
    visuals.widgets.hovered.weak_bg_fill = panel_hover;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.hovered.corner_radius = corner_radius;

    // Active (pressed): solid cyan with dark ink for max contrast
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.weak_bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, ON_ACCENT);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.active.corner_radius = corner_radius;

    // Open (expanded combo/menu)
    visuals.widgets.open.bg_fill = panel;
    visuals.widgets.open.weak_bg_fill = panel;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, SEPARATOR_BG);
    visuals.widgets.open.corner_radius = corner_radius;

    // Selection: selected tab / highlighted text = cyan fill, dark ink label
    visuals.selection.bg_fill = ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);

    // Sliders fill their track with the accent up to the handle.
    visuals.slider_trailing_fill = true;

    ctx.set_visuals(visuals);
}

/// A rounded, subtly-bordered panel — the building block of the "command
/// centre" look. Chain `.inner_margin(..)` to override the default padding.
pub fn card(fill: Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, SEPARATOR_BG))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin::same(14))
}

/// A section heading preceded by a short vertical accent bar.
pub fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, CornerRadius::same(2), ACCENT);
        ui.add_space(9.0);
        ui.label(egui::RichText::new(title).size(16.0).strong().color(FG));
    });
}

/// egui bundles only Latin + emoji glyphs, so Traditional Chinese text renders as
/// empty "tofu" boxes out of the box. Load a CJK-capable system font (Microsoft
/// JhengHei on Windows) and append it as a fallback for the proportional and
/// monospace families. Best-effort: if no candidate font is present we leave
/// egui's defaults untouched rather than failing.
#[cfg(all(not(target_arch = "wasm32"), any(windows, target_os = "linux")))]
fn install_cjk_fonts(ctx: &Context) {
    use std::sync::Arc;

    #[cfg(windows)]
    let candidates: &[(&str, u32)] = &[
        (r"C:\Windows\Fonts\msjh.ttc", 0), // Microsoft JhengHei (Traditional Chinese)
        (r"C:\Windows\Fonts\msjhl.ttc", 0), // JhengHei Light
        (r"C:\Windows\Fonts\msyh.ttc", 0), // Microsoft YaHei (Simplified fallback)
        (r"C:\Windows\Fonts\simsun.ttc", 0), // SimSun
    ];
    #[cfg(target_os = "linux")]
    let candidates: &[(&str, u32)] = &[
        ("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc", 1),
        ("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc", 1),
        ("/usr/share/fonts/truetype/wqy/wqy-microhei.ttc", 0),
    ];

    for (path, index) in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut font = egui::FontData::from_owned(bytes);
            font.index = *index;

            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert("cjk".to_owned(), Arc::new(font));
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts
                    .families
                    .entry(family)
                    .or_default()
                    .push("cjk".to_owned());
            }
            ctx.set_fonts(fonts);
            return;
        }
    }
}

#[cfg(not(all(not(target_arch = "wasm32"), any(windows, target_os = "linux"))))]
fn install_cjk_fonts(_ctx: &Context) {}
