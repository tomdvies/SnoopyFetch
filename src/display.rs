use crate::system_info::SystemInfo;
use crate::DisplayConfig;
use colored::*;
use regex::Regex;

const HEADER_COLOR: &str = "blue"; // Bright green, can be easily changed

fn format_uptime(seconds: u64) -> String {
    let days = seconds / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes) 
    } else {
        format!("{}m", minutes)
    }
}

fn strip_ansi_length(s: &str) -> usize {
    let re = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    let clean = re.replace_all(s, "");
    clean.as_ref().chars().count()
}

pub fn print_system_info(info: &SystemInfo, config: &DisplayConfig) {
    // Split ASCII art and system info into lines
    let logo_str = if config.art == "snoopy" { get_ascii_logo_snoopy() } else { get_ascii_logo_tree() };

    let logo_lines: Vec<String> = logo_str
        .to_string()
        .lines()
        .map(String::from)
        .collect();

    // Create system info lines with aligned format
    let mut info_lines = Vec::new();
    
    if config.show_hostname {
        info_lines.push(format!("{}@{}", whoami::username().green().bold(), info.hostname.bold().green()));
    }
    
    if config.show_os {
        info_lines.push(format!("{:<8} {}", "OS".color(HEADER_COLOR).bold(), info.os_name));
    }
    
    if config.show_kernel {
        info_lines.push(format!("{:<8} {}", "Kernel".color(HEADER_COLOR).bold(), info.kernel_version));
    }
    
    if config.show_cpu {
        info_lines.push(format!("{:<8} {}", "Arch".color(HEADER_COLOR).bold(), info.cpu_arch));
    }
    
    if config.show_memory {
        info_lines.push(format!("{:<8} {}MB / {}MB", "Memory".color(HEADER_COLOR).bold(), info.memory_used/1024, info.memory_total/1024));
    }
    
    if config.show_uptime {
        info_lines.push(format!("{:<8} {}", "Uptime".color(HEADER_COLOR).bold(), format_uptime(info.uptime)));
    }
    
    if config.show_shell {
        info_lines.push(format!("{:<8} {}", "Shell".color(HEADER_COLOR).bold(), info.shell));
    }
    
    if config.show_packages {
        info_lines.push(format!("{:<8} (brew) {}", "Packages".color(HEADER_COLOR).bold(), info.brew_packages));
    }

    let logo_width = logo_lines
        .iter()
        .map(|line| strip_ansi_length(line))
        .max()
        .unwrap_or(0);

    // Print logo and info side by side
    let max_lines = logo_lines.len().max(info_lines.len());
    for i in 0..max_lines {
        let logo_line = logo_lines.get(i).map_or("".to_string(), |s| s.to_string());
        let info_line = info_lines.get(i).map_or("".to_string(), |s| s.to_string());
        
        // Add padding between logo and info (4 spaces)
        if config.info_on_left {
            println!(
                "{}{}{}",
                info_line,
                " ".repeat(4 + (info_lines.iter().map(|l| strip_ansi_length(l)).max().unwrap_or(0) - strip_ansi_length(&info_line))),
                logo_line
            );
        } else {
            println!(
                "{}{}{}",
                logo_line,
                " ".repeat(4 + (logo_width - strip_ansi_length(&logo_line))),
                info_line
            );
        }
    }

}

// fn get_ascii_logo() -> String {
//     format!("{}\n{}\n{}\n{}\n{}\n{}",
//         "  \\,`/ / ".yellow(),
//         " _)..  `_".yellow(),
//         "( __  -\\".yellow(),
//         "    '`.".yellow(),
//         "   ( \\>_-_,".yellow(),
//         "   _||_ ~-/".yellow()
//     )
// }

fn get_ascii_logo_snoopy() -> String {
    // format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡴⠚⠉⠉⠉⠉⠓⠦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁⠀⠀⠀⠀⠀⠀⠀⢀⣼⡷⠶⠖⠶⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⠋⠀⠀⠀⠀⠀⠀⠀⠀⣠⡿⠉⠀⠀⠀⠀⠈⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠁⠀⢀⡴⠒⠲⣤⣀⣀⣰⣿⠀⠀⠀⠀⠀⠀⢠⡟⠉⠙⠓⠒⠦⣤⣀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠁⠀⢀⡞⠀⠀⠀⢸⣤⣽⠼⣿⣄⠀⠀⠀⢀⣴⡿⠃⠀⠀⠀⠀⠀⠀⠙⢷⡄⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡇⠀⠀⣸⠀⠀⠀⠀⢸⠃⠀⠀⢸⠟⠛⠛⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡄⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡇⠀⠀⠸⣄⣀⣀⣠⠎⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⡀⠀⠀⠈⠉⠉⠀⠀⠀⢀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡿⠐⢦",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠳⣄⠀⠀⠀⠀⠀⠀⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣷⡆⡸",
    // "⠀⣀⣀⠀⠀⢀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⢤⡀⠀⠀⠀⢀⡇⠀⠀⠀⠀⠀⢀⣀⡀⠀⣀⣀⠀⠀⠀⢀⣠⠔⠋⠙⠛⠋⠁",
    // "⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣾⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⢢⡀⢸⣇⠀⠶⣚⡭⠿⠛⠛⠛⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⣤⣤⣄⣀⣀⣀⣀⣀⣀⣠⣤⣴⣿⣿⣿⣿⣾⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠿⢿⣿⣿⡿⠿⠛⣩⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⣿⡇⠀⠀⣠⠶⠒⠒⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠈⠉⣁⣤⣶⣿⣿⠿⠿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⠛⠛⠉⠉⠁⠀⣼⠇⠀⠀⣀⣸⣷⣶⣶⣧⠀⠀⣴⣯⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠉⠉⠉⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⠃⠀⢀⡴⠚⠛⢦⣄⣀⡄⠀⠀⠀⢻⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⠏⠀⢀⡞⠀⣠⢤⡬⣿⢹⣧⣀⡴⠒⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠿⠟⠃⠀⠀⡜⠀⠀⠀⠀⠀⢸⡟⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠇⠀⠀⠀⠀⠀⡇⠀⠀⠐⠉⠉⣿⠃⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣼⠀⠀⠒⠒⠒⠶⣷⠀⠀⠀⠀⠀⣿⢰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠤⠔⠒⠉⠁⠀⠠⣾⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡇⣼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    // "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠻⠶⠶⠶⠶⠤⣤⣤⣤⣤⣤⣤⣾⣯⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀")
    // format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠄⠀⠠⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⢀⡴⠋⠉⢢⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠠⠊⠰⣲⣾⡀⠀⢀⡜⠉⠉⠒⢤⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⢇⠀⡠⠀⠰⠉⠉⠉⠀⠀⠀⠀⠀⢡⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⢠⠁⠀⠀⠀⠀⠀⠀⠀⠀⣼⢑".white(),
    //     "⣤⣤⣤⣤⣤⣤⣤⡀⠀⠀⠀⠀⠀⠑⠀⡀⢀⢀⢤⠤⠤⠤⠤⠤⠐⠈⠉⠁".white(),
    //     "⣴⣿⡿⢿⣿⣿⣿⣿⣿⣶⣶⣦⣤⣶⣶⡾⠿⡿⠁⡀⠤⡀⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠡⠶⠟⠋⠛⠛⠛⠛⠛⠛⠉⠉⠀⣼⠀⠀⠚⠚⠃⠐⢣⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⠇⠠⢊⣑⡦⣂⡀⠇⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠊⠁⠀⠃⠀⠤⡟⡃⠀⠀⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠒⣾⠀⠈⠑⠃⠀⠀⣧⠁⠀⠀⠀⠀⠀⠀⠀⠀".white(),
    //     "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠁⠒⠒⠒⠒⠚⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀".white()
    // )
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "⠀⠀    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀".white(),
        "⠀⠀⠠⠄⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀".white(),
        "⠀⠀⠀⠀⠀⠀⠁⠀⠀⢀⡠⠤⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⠤⠤⠀⠀⠀⠀".white(),
        "⠀⠀⠘⠃⠀⠀⣀⡠⠔⠁⠀⠀⠀⠈⢦⠀⠀⠀⠀⢠⡖⠋⠉⠉⠻⡒⠈⠁⠀⠀".white(),
        "⠀⠀⢠⠔⠉⠁⠀⠀⠄⠀⠀⠀⠠⣴⣦⣧⠀⠀⢠⣿⠃⢀⠀⠀⠀⠸⠈⢂⠀⠀".white(),
        "⠀⢠⠁⠀⠀⠀⠀⠀⠀⠀⠀⢠⢳⣿⣿⣿⣄⠄⠉⠁⡔⠞⠢⡀⠀⠀⠀⠀⢄⡀".white(),
        "⣐⣺⡀⠀⠀⠀⠀⠀⠀⠀⠀⡆⣼⣿⣿⡿⠀⠤⠤⠤⠏⠀⠀⠁⠀⠀⢆⠀⠀⡡".white(),
        "⠈⠋⠈⠒⠦⠤⠤⠒⠒⠒⠒⠧⣻⡿⠟⠳⠤⠤⠤⠤⠤⠤⠔⠑⠒⠊⠀⠁⠂⠁".white()
    )
}

fn get_ascii_logo_tree() -> String {
    format!("{}
{}
{}
{}
{}{}
{}
{}
{}",
        "        ccc8OE88oo".green(),
        // "      C8O8O8Q8PoOb o8oo".green(),
        "   dB69QO8PdUOpugoO9bD".green(),
        " CggbU8OU qOp qOdoUOdcb".green(),
        "      6OuU  /p u gcoUodpP".green(),
        "         \\\\//  /".white(), "douUP".green(),
        "          ||  ||".white(),
        // "          ||  |||".white(),
        "          ||  ||".white(),
        "    _____//   \\\\____".white()
    )
}
