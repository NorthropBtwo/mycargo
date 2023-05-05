use std::env;
use std::fs;
use std::io;
use std::process::Output;
use std::str;
use std::process::Command;


fn main() {

    // get and check arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Incorect number of arguments.");
        return;
    }

    let second_arg = &args[1];
    if second_arg != "new" {
        eprintln!("Second argument must be 'new'.");
        return;
    }

    let project_name = &args[2];
    let target_path = project_name.to_string() + "\\.vscode";

    // create project with cargo new
    create_new_project(project_name).expect("Could not create an new project with cargo.");

    // create .vscode folder to place configuration file in it
    if fs::metadata(&target_path).is_err() {
        fs::create_dir_all(&target_path).expect("Could no create vscode folder.");
    }

    // create configuration file for debugging and launching rust programm  with shourtcut (mine: F5 to lauch and F7 to debug)
    fs::write(target_path.clone() + "\\" + "tasks.json", get_task_file_content()).expect("tasks.json could not be created");
    fs::write(target_path.clone() + "\\" + "launch.json", get_launch_file_content(project_name)).expect("launch.json could not be created");

   // start vs code
    start_vs_code(project_name).expect("Could not start vs code.");
}


/// calls "cargo new project_name"
fn create_new_project(project_name: &str) -> io::Result<Output> {
    let command = Command::new("cargo")
        .args(&["new", project_name])
        .output();

    match &command {
        Ok(output) => {
            if output.status.success() {
                /*let stdout = std::str::from_utf8(&output.stdout).unwrap();*/
                /*println!("Command executed successfully:\n{}", stdout);*/
            } else {
                let stderr = std::str::from_utf8(&output.stderr).unwrap();
                eprintln!("Command failed with error:\n{}", stderr);
            }
        }
        Err(err) => eprintln!("Error executing command: {}", err),
    }

    return command;
}

/// opens ms code by calling "code .//project_name"
fn start_vs_code(project_name: &str) -> io::Result<Output> {
    let command = Command::new("cmd")
        .args(&["/C".to_string(), "code".to_string(), ".".to_string() + "\\" + project_name])
        .output();

    match &command {
        Ok(output) => {
            if output.status.success() {
                /*let stdout = std::str::from_utf8(&output.stdout).unwrap();*/
                /*println!("Command executed successfully:\n{}", stdout);*/
            } else {
                let stderr = std::str::from_utf8(&output.stderr).unwrap();
                eprintln!("Command failed with error:\n{}", stderr);
            }
        }
        Err(err) => eprintln!("Error executing command: {}", err),
    }

    return command;
}

/// returns content of the task.json file
fn get_task_file_content() -> &'static str {
    r#"{ 
    "version": "2.0.0",
    "tasks": [
        {
            "type": "cargo",
            "command": "build",
            "problemMatcher": [
                "$rustc"
            ],
            "group": "build",
            "label": "rust: cargo build",
            "presentation": {
            "clear": true
            }
        }
    ,
        {
            "type": "cargo",
            "command": "run",
            "args": [],
            "problemMatcher": [
                "$rustc"
            ],
            "group": {
                "kind": "build",
                "isDefault": true
                },
            "label": "rust: cargo run",
            "presentation": {
                "clear": true
            }
        }
    ]
}"#
}

/// returns content of the launch.json file
fn get_launch_file_content(project_name: &str) -> String {
r#"{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable '{project_name}'",
            "cargo": {
                "args": [
                    "build",
                    "--bin={project_name}",
                    "--package={project_name}"
                ],
                "filter": {
                    "name": "{project_name}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable '{project_name}'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin={project_name}",
                    "--package={project_name}"
                ],
                "filter": {
                    "name": "{project_name}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}"#.replace("{project_name}", project_name)
}