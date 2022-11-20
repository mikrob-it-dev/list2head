use crate::data_model::{self, checklist, checklist_step, checklist_section};
use crate::{app_config::AppConfig, app_constants::AppConstants, log_utils};
use egui::FontFamily::Proportional;
use egui::{Align2, Button, Context, FontData, FontId, Label, TextStyle, Ui, Vec2};
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
    pub checklist: checklist,
    pub checklist_current_section: usize,
    pub checklist_current_step: usize,
}

impl MyApp {
    pub fn default() -> Self {
        Self {
            is_license_info_shown: false,
            checklist: data_model::checklist::load_checklist(),
            checklist_current_section: 1,
            checklist_current_step: 1,
        }
    }
}

// TODO: continuous refresh

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, FontId::new(20.0, Proportional)),
            (egui::TextStyle::Body, FontId::new(20.0, Proportional)),
            (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
            (egui::TextStyle::Small, FontId::new(20.0, Proportional)),
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
            ui_add_step_list(ui, self, ctx);
            ui_add_controls(ui, self, _frame, ctx);

            ui_add_dev_version_info(self, ctx);
            ui_add_license_info(self, ctx);
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.label("test");
        // });
    }
}

fn ui_add_step_list(ui: &mut Ui, my_app: &mut MyApp, ctx: &Context) {
    egui::CollapsingHeader::new(format!("Checklist: {}", &my_app.checklist.name))
        .default_open(true)
        .show(ui, |ui| {
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(100.0).at_least(40.0))
                .column(Size::initial(600.0).at_least(40.0))
                .column(Size::remainder().at_least(100.0))
                .resizable(true);

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
                })
                .body(|mut body| {
                    // for row_index in 0..20 {
                    //     let row_height = 30.0;
                    //     body.row(row_height, |mut row| {
                    //         row.col(|ui| {
                    //             ui.label(row_index.to_string());
                    //         });
                    //         row.col(|ui| {
                    //             ui.label(
                    //                 char::from_u32(0x1f550 + row_index as u32 % 24)
                    //                     .unwrap()
                    //                     .to_string(),
                    //             );
                    //         });
                    //         row.col(|ui| {
                    //             ui.style_mut().wrap = Some(false);
                    //             ui.heading("row");
                    //         });
                    //     });
                    // }

                    let row_height = 45.0;

                    for section in &my_app.checklist.sections {

                        if my_app.checklist_current_section >= section.order {
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{}-{}",
                                        section.order.to_string(),
                                        0,
                                    ));
                                });
                                row.col(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(section.name.to_owned());
                                        ui.label(section.section_description); 
                                    });
                                });
                            });
                        };

                        for step in &section.checklist_steps {

                            if section.order < my_app.checklist_current_section.try_into().unwrap()
                                || (section.order == my_app.checklist_current_section  && step.order <= my_app.checklist_current_step.try_into().unwrap())
                            {
                                body.row(row_height, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{}-{}",
                                            section.order.to_string(),
                                            step.order.to_string(),
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(step.text.to_owned());
                                            ui.label(step.comment.to_owned());
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
                                });
                            }
                        }
                    }
                });
        });

    ui.add_space(15.0);
}

fn ui_add_controls(ui: &mut Ui, my_app: &mut MyApp, _frame: &mut eframe::Frame, ctx: &Context) {
    egui::Area::new("Results")
        .anchor(Align2::LEFT_BOTTOM, [10.0, -10.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let advance_step_button = ui.button("Step ahead");
                if advance_step_button.clicked() {
                    my_app.checklist_current_step += 1;

                    if my_app.checklist_current_step == my_app.checklist.sections[my_app.checklist_current_section-1].checklist_steps.iter().count() + 1 {
                        my_app.checklist_current_step = 1;
                        my_app.checklist_current_section += 1;
                    };
                }

                let start_over_button = ui.button("Start over");
                if start_over_button.clicked() {
                    *my_app = MyApp::default();
                }
            })
        });

    ui.add_space(15.0);
}

fn ui_add_dev_version_info(my_app: &mut MyApp, ctx: &Context) {
    egui::Area::new("dev, version info")
        .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("developed by:");
                ui.hyperlink_to(
                    format!("{}", AppConstants::APP_DEVELOPER),
                    AppConstants::APP_DEVELOPER_WEBSITE,
                );
                ui.label(format!("|  version: {}   |", AppConstants::APP_VERSION));
                let license_button_handle = ui.button("License: MIT");
                if license_button_handle.clicked() {
                    my_app.is_license_info_shown = !my_app.is_license_info_shown;
                }
            });
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
