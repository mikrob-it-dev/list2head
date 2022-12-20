use crate::app_constants::AppConstants;
use crate::app_model::EguiApp;
use crate::data_model::{self, KeyboardInstruction, StepResult};
use crate::log_utils;
use egui::FontFamily::Proportional;
use egui::{Align, Align2, Context, FontId, Label, Ui, Vec2};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Body,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Button,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Small,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // // egui::Window::new(AppConstants::APP_NAME)
            //     .fixed_pos(Pos2::new(10.0, 10.0))
            //     .collapsible(false)
            //     .auto_sized()
            //     .default_width(500.0)
            //     .show(ctx, |ui| {

            // keyboard input
            let keyboard_instruction = register_keyboard_instructions(self, ui);

            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(25.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui_add_controls(ui, self);

                        ui.separator();

                        ui.collapsing("Checklist Details", |ui| {
                            let vertical_scrollable = egui::ScrollArea::both();
                            vertical_scrollable.enable_scrolling(true)
                            .max_height(300.0)
                            .show(ui, |ui| {
                                ui.vertical(|ui|{
                                ui.label(self.selected_checklist.checklist_description.to_owned());

                                ui.label("\nChecklist sections:");
                                for checklist_section in &self.selected_checklist.sections {
                                    ui.label(format!(
                                        "{}. {}",
                                        checklist_section.order, checklist_section.name
                                    ));
                                }
                            });
                            });
                        });

                        ui.separator();

                        if !self.selected_checklist.name.eq("N/A") {
                            ui_add_checklist_table(ui, self, keyboard_instruction);
                        }
                    });
                    strip.cell(|ui| {
                        ui_add_dev_version_info(ui, self);
                    });
                });
        });

        ui_add_license_info(self, ctx);
    }
}

fn ui_add_checklist_table(
    ui: &mut Ui,
    my_app: &mut EguiApp,
    keyboard_instruction: KeyboardInstruction,
) {
    let mut step_ahead = false;
    let mut skip_step = false;
    let mut start_section = false;
    let mut skip_section = false;

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(100.0))
        .column(Column::initial(600.0))
        .column(Column::initial(50.0))
        .column(Column::remainder())
        .resizable(true)
        .stick_to_bottom(true);

    table
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading("Step #");
            });
            header.col(|ui| {
                ui.heading("Text");
            });
            header.col(|ui| {
                ui.heading("Result");
            });
            header.col(|ui| {
                ui.heading("Action");
            });
        })
        .body(|mut body| {
            let cloned_checklist_section = &my_app.selected_checklist.sections.clone();
            let my_app_clone = my_app.clone();

            // TODO: section bg colors
            for section in &mut my_app.selected_checklist.sections {
                if (my_app.checklist_position.section >= section.order
                    && section.order >= my_app.checklist_position.start_section.try_into().unwrap())
                {
                    body.row(
                        calculate_row_height(&section.name, &section.section_description),
                        |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}-{}", section.order.to_string(), 0,));
                            });
                            row.col(|ui| {
                                ui.vertical(|ui| {
                                    ui.visuals_mut().override_text_color =
                                        Some(egui::Color32::LIGHT_BLUE);
                                    ui.label(section.name.to_owned());
                                    ui.label(section.section_description.to_owned());
                                });
                            });
                            if my_app.checklist_position.section == section.order
                                && my_app.checklist_position.step == 0
                            {
                                row.col(|_ui| {});
                                row.col(|ui| {
                                    let step_ahead_button = ui.button("Start");
                                    if step_ahead_button.clicked() {
                                        my_app.checklist_position.step += 1;

                                        if my_app.checklist_position.step
                                            == cloned_checklist_section
                                                [my_app.checklist_position.section - 1]
                                                .checklist_steps
                                                .iter()
                                                .count()
                                                + 1
                                            || keyboard_instruction
                                                == KeyboardInstruction::StartSection
                                        {
                                            // TODO: improve... starting at 0 to let section jump in first
                                            start_section = true;
                                        };
                                    }

                                    let skip_section_button = ui.button("Skip");
                                    if skip_section_button.clicked() {
                                        skip_section = true;
                                    }
                                });
                            } else {
                                row.col(|_ui| {});
                                row.col(|_ui| {});
                            }
                        },
                    );
                };

                for step in &mut section.checklist_steps {
                    if (section.order < my_app.checklist_position.section.try_into().unwrap()
                        && section.order
                            >= my_app.checklist_position.start_section.try_into().unwrap())
                        || (section.order == my_app.checklist_position.section
                            && step.order <= my_app.checklist_position.step.try_into().unwrap())
                    {
                        body.row(
                            calculate_row_height(&step.text, &step.comment),
                            |mut row| {
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{}-{}",
                                        section.order.to_string(),
                                        step.order.to_string(),
                                    ));
                                });
                                row.col(|ui| {
                                    ui.vertical(|ui| {
                                        let text_label =
                                            egui::Label::new(step.text.to_owned()).wrap(true);
                                        ui.add(text_label);

                                        let comment_label =
                                            egui::Label::new(step.comment.to_owned()).wrap(true);

                                        ui.add(comment_label);
                                    });
                                });

                                row.col(|ui| {
                                    if step.result == data_model::StepResult::Correct {
                                        ui.visuals_mut().override_text_color =
                                            Some(egui::Color32::DARK_GREEN);
                                    }
                                    if step.result == data_model::StepResult::Incorrect {
                                        ui.visuals_mut().override_text_color =
                                            Some(egui::Color32::DARK_RED);
                                    }
                                    ui.label(match &step.result {
                                        StepResult::Correct => "OK",
                                        StepResult::Incorrect => "NOK",
                                        StepResult::Skipped => "Skipped",
                                        StepResult::Unattempted => "Unattempted",
                                    });
                                });

                                row.col(|ui| {
                                    if step.result == StepResult::Unattempted {
                                        let result_ok_button =
                                            egui::Button::new("OK").fill(egui::Color32::DARK_GREEN);

                                        let result_ok_button_handle = ui.add(result_ok_button);
                                        if result_ok_button_handle.clicked()
                                            || (keyboard_instruction
                                                == KeyboardInstruction::RegisterOkResult
                                                && section.order
                                                    == my_app.checklist_position.section
                                                && step.order
                                                    == my_app.checklist_position.step as u32)
                                        {
                                            step.result = StepResult::Correct;
                                        }

                                        let result_nok_button =
                                            egui::Button::new("NOK").fill(egui::Color32::DARK_RED);

                                        let result_nok_button_handle = ui.add(result_nok_button);
                                        if result_nok_button_handle.clicked()
                                            || (keyboard_instruction
                                                == KeyboardInstruction::RegisterNokResult
                                                && section.order
                                                    == my_app.checklist_position.section
                                                && step.order
                                                    == my_app.checklist_position.step as u32)
                                        {
                                            step.result = StepResult::Incorrect;
                                        }
                                    }

                                    if section.order == my_app.checklist_position.section
                                        && step.order == (my_app.checklist_position.step as u32)
                                        && !my_app_clone.is_end_of_checklist_reached()
                                    {
                                        let button_action =
                                            if step.result == StepResult::Unattempted {
                                                "Skip"
                                            } else {
                                                "Continue"
                                            };
                                        let step_ahead_button = ui.button(button_action);
                                        if step_ahead_button.clicked() {
                                            if button_action.eq("Skip") {
                                                skip_step = true;
                                            }
                                            if button_action.eq("Continue") {
                                                step_ahead = true;
                                            }
                                        }
                                    }
                                });
                            },
                        );
                    }
                }
            }
        });

    if step_ahead || keyboard_instruction == KeyboardInstruction::StepAhead {
        my_app.advance_checklist_step(false);
    }

    if skip_step || keyboard_instruction == KeyboardInstruction::SkipStep {
        my_app.advance_checklist_step(true);
    }

    if start_section || keyboard_instruction == KeyboardInstruction::StartSection {
        my_app.advance_checklist_section(false);
    }

    if skip_section || keyboard_instruction == KeyboardInstruction::SkipSection {
        my_app.advance_checklist_section(true);
    }
}

fn ui_add_dev_version_info(ui: &mut Ui, my_app: &mut EguiApp) {
    let layout = egui::Layout::right_to_left(Align::Center);
    ui.with_layout(layout, |ui| {
        {
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    format!("{}", AppConstants::APP_DEVELOPER),
                    AppConstants::APP_DEVELOPER_WEBSITE,
                );
                ui.label("developed by:");
                ui.label(format!("|  version: {}  |", AppConstants::APP_VERSION));
                let license_button_handle = ui.button("License: MIT");
                if license_button_handle.clicked() {
                    my_app.is_license_info_shown = !my_app.is_license_info_shown;
                }
            })
        }
    });
}

fn ui_add_controls(ui: &mut Ui, my_app: &mut EguiApp) {
    egui::CollapsingHeader::new("Controls")
        .default_open(true)
        .show(ui, |ui| {
            let checklist_select_combobox = egui::ComboBox::new("test_id", "Selected checklist")
                .selected_text(format!("{}", my_app.selected_checklist.name))
                .width(400.0)
                .show_ui(ui, |ui| {
                    for checklist in &my_app.checklists {
                        ui.selectable_value(
                            &mut my_app.selected_checklist,
                            checklist.to_owned(),
                            &checklist.name,
                        );
                    }
                });

            ui.horizontal(|ui| {
                let start_over_button =
                    egui::Button::new("Start over").fill(egui::Color32::DARK_RED);

                let start_over_button_handle = ui.add(start_over_button);

                ui.label(" from section: ");
                ui.text_edit_singleline(&mut my_app.checklist_start_section_string);

                if start_over_button_handle.clicked()
                    || checklist_select_combobox.response.clicked()
                {
                    let start_section_parsed: u32 = my_app
                        .checklist_start_section_string
                        .parse::<u32>()
                        .unwrap();
                    my_app.checklist_position.start_section = start_section_parsed;
                    my_app.reset_checklist(start_section_parsed.try_into().unwrap());
                }

                ui.label("  |  ");

                let edit_checklist_button = egui::Button::new("Edit checklist in Notepad");

                let edit_checklist_button_handle = ui.add_enabled(true, edit_checklist_button);
                if edit_checklist_button_handle.clicked() {
                    match std::process::Command::new("notepad")
                        .arg(&my_app.selected_checklist.checklist_path)
                        .output()
                    {
                        Ok(_) => {}
                        Err(_) => {
                            log::error!("Unable to open notepad to edit current checklist")
                        }
                    }
                };

                let button = egui::Button::new("Application logs");
                let button_handle = ui.add(button);

                if button_handle.clicked() {
                    log_utils::open_logs_externally();
                }
            });

            egui::CollapsingHeader::new("Keyboard shortcuts")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("[A] - Register result (OK)");
                    ui.label("[D] - Register result (NOK)");
                    ui.label("[W] - Skip section");
                    ui.label("[Space] - Start section (on unattempted section)");
                    ui.label("[Space] - Advance one step (on step with result)");
                    ui.label("[Space]- Skip step (on unattepmpted step)");
                });
        });
}

fn ui_add_license_info(my_app: &mut EguiApp, ctx: &Context) {
    egui::Window::new("License")
        .collapsible(false)
        .open(&mut my_app.is_license_info_shown)
        .anchor(Align2::LEFT_TOP, [10.0, 10.0])
        .min_width(800.0)
        .min_height(800.0)
        .vscroll(true)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(Label::new(AppConstants::LICENSE_TEXT).wrap(true));
        });
}

fn calculate_row_height(text: &String, comment: &String) -> f32 {
    let step_number_of_lines: f32 = (text.lines().count() + comment.lines().count()) as f32;
    step_number_of_lines * AppConstants::FONT_SIZE * 1.2 // calibrated constant, ugly
}

fn register_keyboard_instructions(egui_app: &mut EguiApp, ui: &mut Ui) -> KeyboardInstruction {
    let events = ui.input().events.clone();
    for event in &events {
        match event {
            egui::Event::Key {
                key,
                pressed,
                modifiers: _,
            } => {
                if key == &egui::Key::Space && *pressed {
                    if egui_app.checklist_position.step != 0 {
                        if egui_app.selected_checklist.sections
                            [egui_app.checklist_position.section - 1]
                            .checklist_steps[egui_app.checklist_position.step - 1]
                            .result
                            == StepResult::Unattempted
                        {
                            return KeyboardInstruction::SkipStep;
                        } else {
                            return KeyboardInstruction::StepAhead;
                        }
                    } else {
                        return KeyboardInstruction::StartSection;
                    }
                }

                if key == &egui::Key::W && *pressed {
                    return KeyboardInstruction::SkipSection;
                }
                if key == &egui::Key::A && *pressed {
                    return KeyboardInstruction::RegisterOkResult;
                }
                if key == &egui::Key::D && *pressed {
                    return KeyboardInstruction::RegisterNokResult;
                }
            }
            _ => {
                return KeyboardInstruction::None;
            }
        }
    }

    return KeyboardInstruction::None;
}
