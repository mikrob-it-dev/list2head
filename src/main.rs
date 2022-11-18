#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::log_utils::start_logging;
use app_constants::AppConstants;
use app_gui::MyApp;
use eframe::CreationContext;
use egui::Vec2;
use std::{fs::create_dir, io::ErrorKind, thread, time};

mod app_config;
mod app_constants;
mod app_gui;
mod data_model;
mod log_utils;
mod windows_os_utils;

fn main() {
    start_logging();
    log::info!("----------- App started -----------");
    // env::set_var("RUST_BACKTRACE", "1");

    let my_app = MyApp::default();
    let mut secs_since_last_update = 0;

    match create_dir("data") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                log::info!("Already existing /data folder used for storing fetched images");
            }
            _ => {
                log::error!("Could not create /data file");
            }
        },
    };

    let icon = image::open("resources/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    // let options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(900.0, 600.0)),
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
