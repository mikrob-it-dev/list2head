#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{data_model::load_checklists, log_utils::start_logging};
use app_constants::AppConstants;
use app_model::EguiApp;
use egui::Vec2;
use std::{fs::create_dir, io::ErrorKind};

mod app_constants;
mod app_gui;
mod app_model;
mod data_model;
mod log_utils;

fn main() {
    start_logging();
    log::info!("----------- App started -----------");
    // env::set_var("RUST_BACKTRACE", "1");

    load_checklists();

    let my_app = EguiApp::default();

    let icon = image::open("resources/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    // let options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(1000.0, 600.0)),
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };

    eframe::run_native(
        AppConstants::APP_NAME,
        options,
        Box::new(|_cc| Box::new(my_app)),
    );
}
