use super::schema::{
    HigherOrderChoiceOption, HigherOrderChoiceSchema, PresetModifier, PresetSchemaNode,
};
use crate::dashboard::components::presets::schema::PresetModifierOperation;
use settings_schema::ChoiceControlType;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn string_modifier(target_path: &str, value: &str) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::String(value.into())),
    }
}
fn num_modifier(target_path: &str, value: &str) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::Number(
            serde_json::Number::from_str(value).unwrap(),
        )),
    }
}
fn bool_modifier(target_path: &str, value: bool) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::Bool(value)),
    }
}

pub fn resolution_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "resolution".into(),
        strings: [(
            "help".into(),
            "選擇過高的解析度（通常是「High (width: 5184)」）可能導致高延遲或黑畫面。".into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("Very Low (width: 3072)", "1536"),
            ("Low (width: 3712)", "1856"),
            ("Medium (width: 4288)", "2144"),
            ("High (width: 5184)", "2592"),
            ("Ultra (width: 5632)", "2816"),
            ("Extreme (width: 6080)", "3040"),
        ]
        .into_iter()
        .map(|(key, value)| HigherOrderChoiceOption {
            display_name: key.into(),
            modifiers: [
                string_modifier(
                    "session_settings.video.transcoding_view_resolution.variant",
                    "Absolute",
                ),
                num_modifier(
                    "session_settings.video.transcoding_view_resolution.Absolute.width",
                    value,
                ),
                bool_modifier(
                    "session_settings.video.transcoding_view_resolution.Absolute.height.set",
                    false,
                ),
                string_modifier(
                    "session_settings.video.emulated_headset_view_resolution.variant",
                    "Absolute",
                ),
                num_modifier(
                    "session_settings.video.emulated_headset_view_resolution.Absolute.width",
                    value,
                ),
                bool_modifier(
                    "session_settings.video.emulated_headset_view_resolution.Absolute.height.set",
                    false,
                ),
            ]
            .into_iter()
            .collect(),
            content: None,
        })
        .collect(),
        default_option_display_name: "Medium (width: 4288)".into(),
        gui: ChoiceControlType::Dropdown,
    })
}

pub fn framerate_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "preferred_framerate".into(),
        strings: HashMap::new(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [60, 72, 80, 90, 120]
            .into_iter()
            .map(|framerate| HigherOrderChoiceOption {
                display_name: format!("{framerate}Hz"),
                modifiers: [num_modifier(
                    "session_settings.video.preferred_fps",
                    &format!("{:?}", framerate as f32),
                )]
                .into_iter()
                .collect(),
                content: None,
            })
            .collect(),
        default_option_display_name: "72Hz".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn codec_preset_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "codec_preset".into(),
        strings: [(
            "help".into(),
            "AV1 僅支援較新的顯示卡（AMD RX 7xxx+、NVIDIA RTX 30xx+、Intel ARC）！"
                .into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [("H264", "H264"), ("HEVC", "Hevc"), ("AV1", "AV1")]
            .into_iter()
            .map(|(key, val_codec)| HigherOrderChoiceOption {
                display_name: key.into(),
                modifiers: [string_modifier(
                    "session_settings.video.preferred_codec.variant",
                    val_codec,
                )]
                .into_iter()
                .collect(),
                content: None,
            })
            .collect(),
        default_option_display_name: "H264".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn encoder_preset_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "encoder_preset".into(),
        strings: [(
            "help".into(),
            "選擇過高的品質可能導致畫面卡頓或靜止！".into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("Speed", "Speed", "P1"),
            ("Balanced", "Balanced", "P3"),
            ("Quality", "Quality", "P5"),
        ]
        .into_iter()
        .map(|(key, val_amd, val_nv)| HigherOrderChoiceOption {
            display_name: key.into(),
            modifiers: [
                string_modifier(
                    "session_settings.video.encoder_config.nvenc.quality_preset.variant",
                    val_nv,
                ),
                string_modifier(
                    "session_settings.video.encoder_config.quality_preset.variant",
                    val_amd,
                ),
            ]
            .into_iter()
            .collect(),
            content: None,
        })
        .collect(),
        default_option_display_name: "Speed".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn foveation_preset_schema() -> PresetSchemaNode {
    const PREFIX: &str = "session_settings.video.foveated_encoding";
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "foveation_preset".into(),
        strings: [(
            "help".into(),
            "注視點編碼會影響畫面邊緣的像素化程度，並大幅降低編解碼器延遲。
不建議完全停用它，因為這可能導致畫面抖動以及過高的編碼/解碼延遲！"
                .into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("Light", 0.80, 0.80, 8.0, 8.0),
            ("Medium", 0.66, 0.60, 6.0, 6.0),
            ("High", 0.45, 0.40, 4.0, 5.0),
        ]
        .into_iter()
        .map(
            |(key, val_size_x, val_size_y, val_edge_x, val_edge_y)| HigherOrderChoiceOption {
                display_name: key.into(),
                modifiers: [
                    bool_modifier(&format!("{PREFIX}.enabled"), true),
                    num_modifier(
                        &format!("{PREFIX}.content.center_size_x"),
                        &val_size_x.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.center_size_y"),
                        &val_size_y.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.edge_ratio_x"),
                        &val_edge_x.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.edge_ratio_y"),
                        &val_edge_y.to_string(),
                    ),
                ]
                .into_iter()
                .collect(),
                content: None,
            },
        )
        .collect(),
        default_option_display_name: "High".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(target_os = "linux")]
pub fn game_audio_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Headset speaker".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "Disabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "Enabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    true,
                )],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "Enabled".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(target_os = "linux")]
pub fn microphone_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Headset microphone".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "Disabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.microphone.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "Enabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.microphone.enabled",
                    true,
                )],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "Enabled".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(not(target_os = "linux"))]
pub fn game_audio_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Headset speaker".into(),
        strings: [(
            "notice".into(),
            "您可以從系統工作列系統匣（右下角）變更預設音訊裝置"
                .into(),
        )]
        .into_iter()
        .collect(),
        flags: HashSet::new(),
        options: vec![
            HigherOrderChoiceOption {
                display_name: "Disabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "System Default".to_owned(),
                modifiers: vec![
                    bool_modifier("session_settings.audio.game_audio.enabled", true),
                    bool_modifier(
                        "session_settings.audio.game_audio.content.device.set",
                        false,
                    ),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "System Default".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(not(target_os = "linux"))]
pub fn microphone_schema() -> PresetSchemaNode {
    let mut microhone_options = vec![HigherOrderChoiceOption {
        display_name: "Disabled".to_owned(),
        modifiers: vec![bool_modifier(
            "session_settings.audio.microphone.enabled",
            false,
        )],
        content: None,
    }];

    if cfg!(windows) {
        for (key, display_name) in [
            ("Automatic", "Automatic"),
            ("VAC", "Virtual Audio Cable"),
            ("VBCable", "VB Cable"),
            ("VoiceMeeter", "VoiceMeeter"),
            ("VoiceMeeterAux", "VoiceMeeter Aux"),
            ("VoiceMeeterVaio3", "VoiceMeeter VAIO3"),
        ] {
            microhone_options.push(HigherOrderChoiceOption {
                display_name: display_name.into(),
                modifiers: vec![
                    bool_modifier("session_settings.audio.microphone.enabled", true),
                    string_modifier(
                        "session_settings.audio.microphone.content.devices.variant",
                        key,
                    ),
                ],
                content: None,
            })
        }
    }

    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Headset microphone".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: microhone_options.into_iter().collect(),
        default_option_display_name: "Disabled".into(),
        gui: ChoiceControlType::Dropdown,
    })
}

pub fn hand_tracking_interaction_schema() -> PresetSchemaNode {
    const HELP: &str = r"Disabled：手部無法模擬按鍵。適合用於 Joy-Con 或其他非原生控制器。
SteamVR Input 2.0：為手部追蹤建立獨立的 SteamVR 裝置。
ALVR bindings：使用 ALVR 手部追蹤按鍵綁定。請參閱 wiki 尋求協助。
";

    const PREFIX: &str = "session_settings.headset.controllers.content";

    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Hand tracking interaction".into(),
        strings: [("help".into(), HELP.into())].into_iter().collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            HigherOrderChoiceOption {
                display_name: "Disabled".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        false,
                    ),
                    bool_modifier(
                        &format!("{PREFIX}.hand_tracking_interaction.enabled"),
                        false,
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "SteamVR Input 2.0".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(&format!("{PREFIX}.hand_skeleton.enabled"), true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        true,
                    ),
                    bool_modifier(
                        &format!("{PREFIX}.hand_tracking_interaction.enabled"),
                        false,
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "ALVR bindings".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        false,
                    ),
                    bool_modifier(&format!("{PREFIX}.hand_tracking_interaction.enabled"), true),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "SteamVR Input 2.0".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn eye_face_tracking_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "Eye and face tracking".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "Disabled".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.headset.face_tracking.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "VRChat Eye OSC".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.face_tracking.enabled", true),
                    string_modifier(
                        "session_settings.headset.face_tracking.content.sink.variant",
                        "VrchatEyeOsc",
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "VRCFaceTracking".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.face_tracking.enabled", true),
                    string_modifier(
                        "session_settings.headset.face_tracking.content.sink.variant",
                        "VrcFaceTracking",
                    ),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "Disabled".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}
