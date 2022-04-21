mod handler;

use std::io::Read;

use egui::{self, Vec2};
use tokio;
use eframe::{self, NativeOptions, run_native};

use crate::handler::Event;

#[tokio::main]
async fn main() {

    let mut name_file = std::fs::OpenOptions::new().read(true).write(true).open("name.txt").expect("cannot open users file");
    let mut minpage_file = std::fs::OpenOptions::new().read(true).write(true).open("minpage.txt").expect("cannot open users file");
    let mut maxpage_file = std::fs::OpenOptions::new().read(true).write(true).open("maxpage.txt").expect("cannot open users file");
    let mut rawdata_file = std::fs::OpenOptions::new().read(true).write(true).open("rawdata.txt").expect("cannot open users file");

    let mut name_buffer = String::new();
    let _readed_name = name_file.read_to_string(&mut name_buffer).unwrap();

    let mut min_page = String::new();
    let _readed_name = minpage_file.read_to_string(&mut min_page).unwrap();

    let mut max_page = String::new();
    let _readed_name = maxpage_file.read_to_string(&mut max_page).unwrap();

    let mut raw_data = String::new();
    let _readed_name = rawdata_file.read_to_string(&mut raw_data).unwrap();

    let app = Event{
        name_buffer,
        min_page,
        max_page,
        raw_data,
        name_file,
        minpage_file,
        maxpage_file,
        rawdata_file,
        page_err: None,
        name_err: false,
        rawdata_err: false,
    };

    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 1000., y: 800. });
    native_option.resizable = false;

    println!("after select macro");
    run_native(Box::new(app), native_option);
}
