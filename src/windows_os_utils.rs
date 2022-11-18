use std::{env::current_dir, ffi::OsStr, iter, os::windows::prelude::OsStrExt, path::PathBuf};

use winapi::{
    ctypes::c_void,
    um::winuser::{
        SetSysColors, SystemParametersInfoW, COLOR_BACKGROUND, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
        SPI_SETDESKWALLPAPER,
    },
};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn build_absolute_path(relative_path_str: &str) -> PathBuf {
    current_dir().unwrap().join(relative_path_str)
}
