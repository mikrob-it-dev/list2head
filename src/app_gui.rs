use crate::data_model::{self, checklist, checklist_section, checklist_step};
use crate::{app_config::AppConfig, app_constants::AppConstants, log_utils};
use egui::FontFamily::Proportional;
use egui::{Align, Align2, Button, Context, FontData, FontId, Label, TextStyle, Ui, Vec2};
use egui_extras::{Size, StripBuilder, TableBuilder};
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Enum {
    Center,
    Fit,
    Stretch,
}

#[derive(Clone)]
pub struct MyApp {
    pub is_license_info_shown: bool,
    pub checklists: Vec<checklist>,
    pub selected_checklist: checklist,
    pub checklist_current_section: usize,
    pub checklist_current_step: usize,
}

impl MyApp {
    pub fn default() -> Self {
        Self {
            is_license_info_shown: false,
            checklists: data_model::load_checklists(),
            checklist_current_section: 1,
            selected_checklist: data_model::load_checklists().get(0).unwrap().to_owned(),
            // TODO: improve... starting at 0 to let section jump in first
            checklist_current_step: 0,
        }
    }
}

impl eframe::App for MyApp {
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

        // TODO: Checklist info to top panel

        egui::CentralPanel::default().show(ctx, |ui| {
            // // egui::Window::new(AppConstants::APP_NAME)
            //     .fixed_pos(Pos2::new(10.0, 10.0))
            //     .collapsible(false)
            //     .auto_sized()
            //     .default_width(500.0)
            //     .show(ctx, |ui| {

            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(25.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        let collapsing_controls_section = egui::CollapsingHeader::new("Controls")
                            .default_open(true)
                            .show(ui, |ui| {
                                egui::ComboBox::new("test_id", "Selected checklist")
                                    .selected_text(format!("{}", self.selected_checklist.name))
                                    .width(400.0)
                                    .show_ui(ui, |ui| {
                                        for checklist in &self.checklists {
                                            ui.selectable_value(
                                                &mut self.selected_checklist,
                                                checklist.to_owned(),
                                                &checklist.name,
                                            );
                                        }
                                    });

                                ui.horizontal(|ui| {
                                    let start_over_button = egui::Button::new("Start over")
                                        .fill(egui::Color32::DARK_RED);

                                    let start_over_button_handle = ui.add(start_over_button);
                                    if start_over_button_handle.clicked() {
                                        self.checklist_current_step = 0;
                                        self.checklist_current_section = 1;
                                    }

                                    let start_over_button = ui.button("Edit checklist in Notepad");
                                    // if start_over_button.clicked() {
                                    //     self.checklist_current_step = 0;
                                    //     self.checklist_current_section = 1;
                                    // }
                                })
                            });

                        ui.separator();

                        ui.collapsing("Checklist Details", |ui| {
                            ui.label(self.selected_checklist.checklist_description.to_owned());
                        });

                        ui.separator();

                        if !self.selected_checklist.name.eq("N/A") {
                            ui_add_checklist_table(ui, self, ctx);
                        }
                    });
                    strip.cell(|ui| {
                        ui_add_dev_version_info(ui, self, ctx);
                    });
                });

            // ui_add_dev_version_info(self, ctx);
        });

        ui_add_license_info(self, ctx);
    }
}

fn ui_add_checklist_table(ui: &mut Ui, my_app: &mut MyApp, ctx: &Context) {
    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Size::initial(100.0))
        .column(Size::initial(600.0))
        .column(Size::initial(50.0))
        .column(Size::remainder())
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
            let my_app_clone = my_app.clone();
            let cloned_checklist_section = &my_app.selected_checklist.sections.clone();

            // TODO: section bg colors
            for section in &mut my_app.selected_checklist.sections {
                let section_row_height = 45.0;

                if my_app.checklist_current_section >= section.order {
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
                            if my_app.checklist_current_section == section.order
                                && my_app.checklist_current_step == 0
                            {
                                row.col(|ui| {});
                                row.col(|ui| {
                                    let step_ahead_button = ui.button("Start section");
                                    if step_ahead_button.clicked() {
                                        my_app.checklist_current_step += 1;

                                        if my_app.checklist_current_step
                                            == cloned_checklist_section
                                                [my_app.checklist_current_section - 1]
                                                .checklist_steps
                                                .iter()
                                                .count()
                                                + 1
                                        {
                                            // TODO: improve... starting at 0 to let section jump in first
                                            my_app.checklist_current_step = 0;
                                            my_app.checklist_current_section += 1;
                                        };
                                    }
                                });
                            } else {
                                row.col(|ui| {});
                                row.col(|ui| {});
                            }
                        },
                    );
                };

                for step in &mut section.checklist_steps {
                    if section.order < my_app.checklist_current_section.try_into().unwrap()
                        || (section.order == my_app.checklist_current_section
                            && step.order <= my_app.checklist_current_step.try_into().unwrap())
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
                                    if step.test_result.eq("OK") {
                                        ui.visuals_mut().override_text_color =
                                            Some(egui::Color32::DARK_GREEN);
                                    }
                                    if step.test_result.eq("NOK") {
                                        ui.visuals_mut().override_text_color =
                                            Some(egui::Color32::DARK_RED);
                                    }
                                    ui.label(&step.test_result);
                                });

                                row.col(|ui| {
                                    if step.test_result.eq("N/A") {
                                        let result_ok_button =
                                            egui::Button::new("OK").fill(egui::Color32::DARK_GREEN);

                                        let result_ok_button_handle = ui.add(result_ok_button);
                                        if result_ok_button_handle.clicked() {
                                            step.test_result = String::from("OK");
                                        }

                                        let result_nok_button =
                                            egui::Button::new("NOK").fill(egui::Color32::DARK_RED);

                                        let result_nok_button_handle = ui.add(result_nok_button);
                                        if result_nok_button_handle.clicked() {
                                            step.test_result = String::from("NOK");
                                        }
                                    }

                                    if section.order == my_app.checklist_current_section
                                        && step.order == (my_app.checklist_current_step as u32)
                                        && !is_end_of_checklist_reached(&my_app_clone)
                                    {
                                        let step_ahead_button =
                                            ui.button(if step.test_result.eq("N/A") {
                                                "Skip"
                                            } else {
                                                "Continue"
                                            });
                                        if step_ahead_button.clicked() {
                                            my_app.checklist_current_step += 1;

                                            if my_app.checklist_current_step
                                                == cloned_checklist_section
                                                    [my_app.checklist_current_section - 1]
                                                    .checklist_steps
                                                    .iter()
                                                    .count()
                                                    + 1
                                            {
                                                // TODO: improve... starting at 0 to let section jump in first
                                                my_app.checklist_current_step = 0;
                                                my_app.checklist_current_section += 1;
                                            };
                                        }
                                    }
                                });
                            },
                        );
                    }
                }
            }
        });
}

fn ui_add_dev_version_info(ui: &mut Ui, my_app: &mut MyApp, ctx: &Context) {
    let layout = egui::Layout::right_to_left(Align::Center);
    ui.with_layout(layout, |ui| {
        {
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    format!("{}", AppConstants::APP_DEVELOPER),
                    AppConstants::APP_DEVELOPER_WEBSITE,
                );
                ui.label("developed by:");
                ui.label(format!("|  version: {}   |", AppConstants::APP_VERSION));
                let license_button_handle = ui.button("License: MIT");
                if license_button_handle.clicked() {
                    my_app.is_license_info_shown = !my_app.is_license_info_shown;
                }
            })
        }
    });
}

fn ui_add_license_info(my_app: &mut MyApp, ctx: &Context) {
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

// TODO: Consider using first / last instead of actual indices
fn is_end_of_checklist_reached(my_app: &MyApp) -> bool {
    let number_of_sections = my_app.selected_checklist.sections.iter().count();
    let last_checklist_step = my_app
        .selected_checklist
        .sections
        .last()
        .unwrap()
        .checklist_steps
        .iter()
        .count();
    if (my_app.checklist_current_section == number_of_sections)
        && (my_app.checklist_current_step == last_checklist_step)
    {
        return true;
    } else {
        return false;
    };
}

fn calculate_row_height(text: &String, comment: &String) -> f32 {
    let step_number_of_lines: f32 = (text.lines().count() + comment.lines().count()) as f32;
    5.0 + step_number_of_lines * AppConstants::FONT_SIZE
}
