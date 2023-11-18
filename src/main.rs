//use clap::Parser;
use std::process::Command;
use std::str;

use ansi_term::Color;
use ansi_term::Style;

//pub const CAT: &str = "\
// /| ､
//(°､ ｡ 7
// |､ ~ヽ
// じしf_,)〳
//";

pub const CAT_0: &str = "               ";
pub const CAT_1: &str = " /| ､          ";
pub const CAT_2: &str = "(°､ ｡ 7        ";
pub const CAT_3: &str = " |､ ~ヽ        ";
pub const CAT_4: &str = " じしf_,)〳    ";

fn main() {
    println!(
        "{}{}     {}",
        Color::Yellow.paint(CAT_0).to_string(),
        format_category("OS:"),
        Color::White.paint(get_os()).to_string()
    );
    println!(
        "{}{}   {}",
        Color::Yellow.paint(CAT_1).to_string(),
        format_category("Host:"),
        Color::White.paint(get_host()).to_string()
    );
    println!(
        "{}{} {}",
        Color::Yellow.paint(CAT_2).to_string(),
        format_category("Kernel:"),
        Color::White.paint(get_kernel()).to_string()
    );
    println!(
        "{}{}    {}",
        Color::Yellow.paint(CAT_3).to_string(),
        format_category("RAM:"),
        Color::White.paint(get_ram_info()).to_string()
    );
    println!(
        "{}{} {}",
        Color::Yellow.paint(CAT_4).to_string(),
        format_category("Uptime:"),
        Color::White.paint(get_uptime()).to_string()
    );
}

fn get_kernel() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");

    let kernel_result = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    let kernel = kernel_result.trim_end();
    kernel.to_string()
}

fn get_ram_info() -> String {
    let output = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to execute command");

    let ram_result = str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    let ram_line = ram_result.lines().skip(1).next().unwrap_or_default();
    let total_ram = ram_line.split_whitespace().nth(1).unwrap_or_default();
    let used_ram = ram_line.split_whitespace().nth(2).unwrap_or_default();

    let formatted_ram = format!("{}B / {}B", used_ram, total_ram);
    formatted_ram
}

fn get_os() -> String {
    let os_release = std::fs::read_to_string("/etc/os-release").expect("Failed to read file");
    let os_line = os_release.lines().skip(1).next().unwrap_or_default();
    let mut os_final = os_line.split('=').nth(1).unwrap_or_default();
    os_final = os_final.trim_matches('"');
    os_final.to_string()
}

fn get_host() -> String {
    let host = std::fs::read_to_string("/sys/firmware/devicetree/base/model")
        .expect("Failed to read file")
        .trim_end()
        .to_string();
    host
}

fn get_uptime() -> String {
    let output = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("Failed to execute command");
    let uptime_result =
        str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    let uptime = uptime_result.replace("up", "");
    uptime.trim().to_string()
}

fn format_category(input: &str) -> String {
    let underline_input = Style::new().underline().paint(input).to_string();
    let bold_underline_input = Style::new().bold().paint(underline_input);
    bold_underline_input.to_string()
}
