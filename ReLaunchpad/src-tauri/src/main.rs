// Prevents additional console window on macOS in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    relaunchpad_lib::run()
}
