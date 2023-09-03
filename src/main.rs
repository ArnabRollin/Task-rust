use serde::Deserialize;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command};

#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    command: String,
}

fn main() {
    // Open the JSON file
    let mut file = File::open("tasks.json").expect("Failed to open file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Deserialize the JSON string into your struct
    let data: Vec<Task> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let mut args = args();
    args.next();

    if args.len() > 0 {
        let do_task = args.nth(0).unwrap();
        let mut arguments: Vec<String> = vec![];

        for arg in args {
            arguments.push(arg);
        }

        for task in data {
            if task.name == do_task {
                println!("executing: {} {:?}", task.command, &arguments);
                exec_command(&task.command, &arguments);
            }
        }
    }
}

fn exec_command(command: &str, arguments: &Vec<String>) {
    let mut commands = command.split("&&").map(str::trim);

    if let Some(cmd) = commands.next() {
        let mut parts = cmd.split_whitespace();
        let command = parts.next().expect("Invalid command");
        let args: Vec<_> = parts.collect();

        let output = Command::new(command)
            .args(args)
            .args(arguments)
            .output()
            .expect("Failed to execute task");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}\n{}", stdout, stderr);
            exit(output.status.code().unwrap_or(1));
        }
    }

    for cmd in commands {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute task");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}\n{}", stdout, stderr);
            exit(output.status.code().unwrap_or(1));
        }
    }
}
