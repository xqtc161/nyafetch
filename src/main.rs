//use clap::Parser;
use std::process::Command;
use std::str;

pub const CAT: &str = "\
 /| ､
(°､ ｡ 7
 |､ ~ヽ
 じしf_,)〳 
";
pub const CAT_1: &str = " /| ､          ";
pub const CAT_2: &str = "(°､ ｡ 7        ";
pub const CAT_3: &str = " |､ ~ヽ        ";
pub const CAT_4: &str = " じしf_,)〳    ";
pub const CAT_PLACE: &str = "               ";
//#[derive(Parser, Debug)]
//#[command(author, version, about, long_about = None)]
//struct Args {
//    #[arg(short)]
//    message: Option<String>,
//}

fn main() {
    println!("{}OS:     {}", CAT_1, get_os());
    println!("{}Host:   {}", CAT_2, get_host());
    println!("{}Kernel: {}", CAT_3, get_kernel());
    println!("{}RAM:    {}", CAT_4, get_ram_info());
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

    // Extracting total RAM information
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
        .output()
        .expect("Failed to execute command");
    let uptime_result = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    let uptime = uptime_result.trim_end();
    uptime.to_string()
}
