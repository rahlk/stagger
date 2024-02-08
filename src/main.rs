use clap::{Arg, ArgAction, Command};
use std::process::Command as ProcessCommand;

fn main() {
    let command: Command = Command::new("stagger")
        .version("0.1.0")
        .author("Rahul Krishna <i.m.ralk@gmail.com")
        .about("A CLI tool to perform various helper tasks on my stagger-juice server.")
        .subcommand(
            Command::new("code")
                .about("Opens a folder in VS Code via remote SSH")
                .arg(
                    Arg::new("folder")
                        .help("Path to the folder to open on vscode.")
                        .short('f')
                        .action(ArgAction::Set)
                        .required(true)
                        .long("folder"),
                ),
        );
    let matches = command.get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("code") {
        if let Some(folder_uri) = sub_matches.get_one::<String>("folder") {
            let full_uri: String =
                format!("vscode-remote://ssh-remote+stagger-juice/{}", folder_uri);
            execute_vscode_command_on_remote_folder_uri(&full_uri);
        }
    }
}

fn execute_vscode_command_on_remote_folder_uri(folder_uri: &str) {
    match ProcessCommand::new("code")
        .arg("--folder-uri")
        .arg(folder_uri)
        .spawn()
    {
        Ok(_) => println!("Successfully opened remote folder in VSCode."),
        Err(e) => eprintln!("Failed to open VSCode with error {}", e),
    }
}
