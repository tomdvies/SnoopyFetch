mod display;
mod system_info;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show hostname information
    #[arg(long, default_value = "false")]
    no_hostname: bool,

    /// Show OS information
    #[arg(long, default_value = "false")]
    no_os: bool,

    /// Show kernel information
    #[arg(long, default_value = "false")]
    no_kernel: bool,

    /// Show CPU information
    #[arg(long, default_value = "false")]
    no_cpu: bool,

    /// Show memory information
    #[arg(long, default_value = "false")]
    no_memory: bool,

    /// Show uptime information
    #[arg(long, default_value = "false")]
    no_uptime: bool,

    /// Show shell information
    #[arg(long, default_value = "false")]
    no_shell: bool,

    /// Show brew packages information
    #[arg(long, default_value = "false")]
    no_packages: bool,
    /// Print information on the right side (default is left)
    #[arg(short, long, default_value = "false")]
    right: bool,
}

#[derive(Debug)]
pub struct DisplayConfig {
    pub info_on_left: bool,
    pub show_hostname: bool,
    pub show_os: bool,
    pub show_kernel: bool,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_uptime: bool,
    pub show_shell: bool,
    pub show_packages: bool,
}

fn main() {
    let args = Args::parse();
    let system_info = system_info::collect_system_info();

    let config = DisplayConfig {
        info_on_left: !args.right,
        show_hostname: !args.no_hostname,
        show_os: !args.no_os,
        show_kernel: !args.no_kernel,
        show_cpu: !args.no_cpu,
        show_memory: !args.no_memory,
        show_uptime: !args.no_uptime,
        show_shell: !args.no_shell,
        show_packages: !args.no_packages,
    };

    display::print_system_info(&system_info, &config);
}
