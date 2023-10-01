use std::env;
use std::process;

pub fn run_command(cmd: &str) -> u32 {
    let os = env::consts::OS;

    let (shell, flag) = match os {
        "windows" => ("cmd", "/C"),
        _ => ("sh", "-c"),
    };

    let mut command = process::Command::new(shell);
    command.arg(flag).arg(cmd);

    let pid = command.spawn().expect("System command failed").id();
    
    return pid
}
