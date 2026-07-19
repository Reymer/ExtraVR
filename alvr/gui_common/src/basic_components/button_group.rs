use crate::{theme, DisplayString};
use egui::{Button, RichText, Stroke, Ui};

// Segmented pill/chip control: every option is a rounded chip — the selected one
// is a solid accent pill, the others are subtle bordered chips.
pub fn button_group_clicked(
    ui: &mut Ui,
    options: &[DisplayString],
    selection: &mut String,
) -> bool {
    let mut clicked = false;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;

        for opt in options {
            let selected = *selection == opt.id;

            let text = RichText::new(opt.display.as_str()).color(if selected {
                theme::ON_ACCENT
            } else {
                theme::FG
            });
            let chip = Button::new(text)
                .fill(if selected {
                    theme::ACCENT
                } else {
                    theme::LIGHTER_BG
                })
                .stroke(if selected {
                    Stroke::NONE
                } else {
                    Stroke::new(1.0, theme::SEPARATOR_BG)
                });

            let res = ui.add(chip);
            if res.clicked() {
                selection.clone_from(&opt.id);
                clicked = true;
            }

            if cfg!(debug_assertions) {
                res.on_hover_text(opt.id.clone());
            }
        }
    });

    clicked
}
