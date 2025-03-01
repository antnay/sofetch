use owo_colors::OwoColorize;


use std::env::consts::ARCH;
use std::env::var;
use std::io::{self, stdout, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread::{self};
// use std::process::Command;
// use std::time::Instant;
// use std::{env::var, path::PathBuf};
// use sysinfo::{CpuExt, System, SystemExt};
use whoami::{devicename, distro, username};

// TODO better formatting
// 56ms, 20ms

// #define BOLD "\x1B[1m"
// #define RED "\x1B[31m"
// #define YEL "\x1B[33m"
// #define GRN "\x1B[32m"
// #define CYN "\x1B[36m"
// #define BLU "\x1B[34m"
// #define MAG "\x1B[35m"
// #define WHT "\x1B[37m"
// #define RESET "\x1B[0m"

pub fn mac_art() -> &'static str {
    r#"
                                ____  _____
          ____ ___  ____ ______/ __ \/ ___/
         / __ `__ \/ __ `/ ___/ / / /\__ \
        / / / / / / /_/ / /__/ /_/ /___/ /
       /_/ /_/ /_/\__,_/\___/\____//____/
   "#
}

pub fn linux_art() -> &'static str {
    r#"
                  /\
                 /  \
                /\   \\
               /      \
              /   ,,   \
             /   |  |  -\
            /_-''    ''-_\
    "#
}

fn main() {
    // let now = Instant::now();

    let art: &str;
    if cfg!(target_os = "macos") {
        art = mac_art();
    } else {
        art = linux_art();
    }

    let shell_handle = thread::spawn(|| {
        let shell_path = PathBuf::from(var("SHELL").unwrap_or_else(|_| "N/A".to_string()));
        shell_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("N/A")
            .to_owned()
    });

    let pkg_count_handle = thread::spawn(|| {
        if cfg!(target_os = "macos") {
            Command::new("sh")
                .arg("-c")
                .arg("find /opt/homebrew/Cellar/* -maxdepth 0 -type d | wc -l")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
                .unwrap_or_else(|_| "N/A".to_string())
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("pacman -Qq | wc -l")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
                .unwrap_or_else(|_| "N/A".to_string())
        }
    });

    let distro_handle = thread::spawn(distro);
    let devicename_handle = thread::spawn(devicename);
    let username_handle = thread::spawn(username);

    const MAX_DATA_LENGTH: usize = 64000;

    //let src_data = "\n        ╭──────────╮\x1B[1m";
    //let src = src_data.as_ptr();

    let art_ptr = art.as_ptr();

    // Create destination buffer (with extra space for appending)
    let mut dst_data = [0u8; 400];
    let dst = dst_data.as_mut_ptr();

    unsafe {
        append_bytes(art_ptr, dst, art.len(), 0);
    }
    io::Stdout::write_all(&mut stdout(), &dst_data).unwrap();

    let len = art.len() + 200;
    let mut formatted_string = String::with_capacity(len);

    //formatted_string += art.white().bold().to_string().as_str();

    formatted_string += "\n        ╭──────────╮\x1B[1m";
    formatted_string += "\n        \x1B[37m│\x1B[31m  \x1B[37m user  │\x1B[31m  ";
    formatted_string += username_handle.join().unwrap().as_str();
    formatted_string += "\n        \x1B[37m│\x1B[33m  \x1B[37m os    │\x1B[33m  ";
    formatted_string += distro_handle.join().unwrap().as_str();
    formatted_string += "\n        \x1B[37m│\x1B[32m  󰌢\x1B[37m host  │\x1B[32m  ";
    formatted_string += devicename_handle.join().unwrap().as_str();
    formatted_string += "\n        \x1B[37m│\x1B[36m  \x1B[37m shell │\x1B[36m  ";
    formatted_string += shell_handle.join().unwrap().as_str();
    formatted_string += "\n        \x1B[37m│\x1B[34m  \x1B[37m arch  │\x1B[34m  ";
    formatted_string += ARCH;
    formatted_string += "\n        \x1B[37m│\x1B[35m  󰏖\x1B[37m pkgs  │\x1B[35m  ";
    formatted_string += pkg_count_handle.join().unwrap().as_str();
    formatted_string += "\n        \x1B[37m╰──────────╯";

    // formatted_string.push_str(&format!("\n{:>13}", " "));
    // formatted_string.push_str(&format!("{}", "●".white()));
    // formatted_string.push_str(&format!("{:>3}", "●".red()));
    // formatted_string.push_str(&format!("{:>3}", "●".yellow()));
    // formatted_string.push_str(&format!("{:>3}", "●".cyan()));
    // formatted_string.push_str(&format!("{:>3}", "●".green()));
    // formatted_string.push_str(&format!("{:>3}", "●".blue()));
    // formatted_string.push_str(&format!("{:>3}", "●".magenta()));
    // formatted_string.push_str(&format!("{:>3}\n", "●".black()));
    // formatted_string.push_str("\n\x1B[0m");

    formatted_string += "\n              ";
    formatted_string += "●";
    formatted_string += "  \x1B[31m●";
    formatted_string += "  \x1B[33m●";
    formatted_string += "  \x1B[32m●";
    formatted_string += "  \x1B[36m●";
    formatted_string += "  \x1B[34m●";
    formatted_string += "  \x1B[35m●";
    formatted_string += "  \x1B[30m●\x1b[0m\n";

    print!("{}", formatted_string);
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}

#[inline]
unsafe fn append_bytes(src: *const u8, dst: *mut u8, count: usize, dst_index: usize) {
    let dst_offset = dst.add(dst_index); // Calculate the destination offset
    std::ptr::copy_nonoverlapping(src, dst_offset, count);
}

// pub fn get_logo(sys: &System) -> Option<String> {
//     sys.name().map(|sys_name| {
//         if sys_name.contains("Debian") {
//             get_logo_by_distro(Deb)
//         } else if sys_name.contains("Ubuntu") {
//             get_logo_by_distro(Ubuntu)
//         } else if sys_name.contains("Fedora") {
//             get_logo_by_distro(Fedora)
//         } else if sys_name.contains("Arch") {
//             get_logo_by_distro(Arch)
//         } else if sys_name.contains("Red Hat") {
//             get_logo_by_distro(Redhat)
//         } else {
//             get_logo_by_distro(Linux)
//         }
//     })
// }
