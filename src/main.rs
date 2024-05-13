use owo_colors::OwoColorize;
use std::process::Command;
// use std::time::Instant;
use std::{env::var, path::PathBuf};
use whoami::{arch, devicename, distro, username};

// TODO color icons
// TODO better formatting

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
                                     ____  _____
               ____ ___  ____ ______/ __ \/ ___/
              / __ `__ \/ __ `/ ___/ / / /\__ \
             / / / / / / /_/ / /__/ /_/ /___/ /
            /_/ /_/ /_/\__,_/\___/\____//____/
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

    // let shell_path = PathBuf::from(var("SHELL").unwrap_or(String::from("N/A")));
    // let shell = shell_path
    //     .file_name()
    //     .expect("Could not get $SHELL path")
    //     .to_str()
    //     .unwrap();
    //
    // let pkg_count = Command::new("sh")
    //     .arg("-c")
    //     .arg("find /opt/homebrew/Cellar/*  -maxdepth 0 -type d | wc -l")
    //     .output()
    //     .expect("Failed to execute command");
    //
    let len = art.len() + 200;
    let mut formatted_string = String::with_capacity(len);

    formatted_string += art.green().bold().to_string().as_str();
    formatted_string += "\n       ╭──────────╮";
    formatted_string.push_str(&format!(
        "\n       │  {:<8}│ {}",
        " user".bold(),
        username().red().bold()
    ));
    formatted_string.push_str(&format!(
        "\n       │  {:<8}│ {}",
        " os".bold(),
        distro().yellow().bold()
    ));
    formatted_string.push_str(&format!(
        "\n       │  {:<8}│ {}",
        "󰌢 host".bold(),
        devicename().green().bold()
    ));
    // formatted_string.push_str(&format!(
    //     "\n       │  {:<8}│ {:<10}",
    //     " shell".bold(),
    //     shell.cyan().bold()
    // ));
    formatted_string.push_str(&format!(
        "\n       │  {:<8}│ {:<5}",
        " arch".bold(),
        arch().to_string().blue().bold()
    ));
    // formatted_string.push_str(&format!(
    //     "\n       │  {:<8}│ {:<5}",
    //     "󰏖 pkgs".bold(),
    //     String::from_utf8_lossy(&pkg_count.stdout)
    //         .trim()
    //         .magenta()
    //         .bold()
    // ));
    formatted_string += "\n       ╰──────────╯";

    // println!("\n{}", MAC.green().bold());
    // println!("{:>13}", "╭───────────╮");
    // println!("│{:>11}│ {}", " user".bold(), username().red().bold());
    // println!("│{:>11}│ {}", "os".bold(), distro().yellow().bold());
    // println!("│{:>11}│ {}", "host".bold(), devicename().green().bold());
    // println!("{:>19} {:<10}", "shell".bold(), shell.cyan().bold());
    // println!(
    //     "{:>19} {:<5}",
    //     "arch".bold(),
    //     arch().to_string().blue().bold()
    // );
    // println!(
    //     "{:>19} {:<5}",
    //     "pkgs".bold(),
    //     String::from_utf8_lossy(&pkg_count.stdout)
    //         .trim()
    //         .magenta()
    //         .bold()
    // );

    // colors
    // println!();
    // print!("{:>19}", " ");
    // print!("{}", "●".white());
    // print!("{:>3}", "●".red());
    // print!("{:>3}", "●".yellow());
    // print!("{:>3}", "●".cyan());
    // print!("{:>3}", "●".green());
    // print!("{:>3}", "●".blue());
    // print!("{:>3}", "●".magenta());
    // print!("{:>3}", "●".black());
    // println!();

    formatted_string.push_str(&format!("\n{:>13}", " "));
    formatted_string.push_str(&format!("{}", "●".white()));
    formatted_string.push_str(&format!("{:>3}", "●".red()));
    formatted_string.push_str(&format!("{:>3}", "●".yellow()));
    formatted_string.push_str(&format!("{:>3}", "●".cyan()));
    formatted_string.push_str(&format!("{:>3}", "●".green()));
    formatted_string.push_str(&format!("{:>3}", "●".blue()));
    formatted_string.push_str(&format!("{:>3}", "●".magenta()));
    formatted_string.push_str(&format!("{:>3}\n", "●".black()));
    formatted_string.push_str("\n");

    print!("{}", formatted_string);
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
