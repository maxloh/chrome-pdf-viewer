#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use url::form_urlencoded;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("", &args[1])
        .finish()[1..]
        .replace("%3A", ":")
        .replace("%5C", "\\")
        .replace("+", "%20");
    Command::new("C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe")
        .arg("--app=".to_owned() + &path)
        .spawn()
        .expect("PDF viewer failed to start");
}
