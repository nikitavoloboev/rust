#![allow(unused_imports)]
#![allow(dead_code)]
#[macro_use]
extern crate log_macro;

use dirs_next::home_dir;
use std::env;
use std::fs::{self, write, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// TODO: capture stdout/stderr and write to file
// below command does this:
// env CARGO_TERM_COLOR=always cargo watch -q -- sh -c "tput reset && cargo run -q" 2>&1 | tee ~/log/cmd.log
// it puts results in ~/log/cmd.log
// TODO: make it into standalone binary that does this nicely and smart
// TODO: below code tries to not make it work with `|` pipe to it but tries by wrapping over command it runs (fails too)
fn main() -> io::Result<()> {
    // get cli args
    let args: Vec<String> = env::args().collect();

    log!(args);

    // ensure command is provided
    if args.len() < 2 {
        println!("Usage: <command> [args]");
        return Ok(());
    }

    // Prepare the command from args
    let command = &args[1];
    let command_args = &args[2..];

    // Execute the command
    let mut child = Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture stdout and stderr
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // Prepare log directory
    let mut log_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    log_dir.push("log");
    fs::create_dir_all(&log_dir)?;

    // Prepare files for stdout and stderr
    let mut stdout_file = File::create(log_dir.join("stdout.log"))?;
    let mut stderr_file = File::create(log_dir.join("stderr.log"))?;

    // Read stdout and stderr in separate threads
    let stdout_handle = std::thread::spawn(move || -> io::Result<()> {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            stdout_file.write_all(line.as_bytes())?;
            stdout_file.write_all(b"\n")?;
        }
        Ok(())
    });

    let stderr_handle = std::thread::spawn(move || -> io::Result<()> {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line?;
            stderr_file.write_all(line.as_bytes())?;
            stderr_file.write_all(b"\n")?;
        }
        Ok(())
    });

    // Wait for threads to finish
    stdout_handle.join().unwrap()?;
    stderr_handle.join().unwrap()?;

    // Wait for the child process to finish
    let _ = child.wait()?;

    Ok(())

    // TODO: below code fails, trying again above
    // let cmd = if let Some(index) = args.iter().position(|arg| arg == "-c") {
    //     let (first, second) = args.split_at(index);
    //     format!("{} {}", first.join(" "), second.join(" "))
    // } else {
    //     args[1..].join(" ")
    // };
    // The command to run
    // let cmd = &args[1..].join(" ");
    // println!("cmd: {:?}", cmd);
    // Execute the command and capture its output
    // let mut child = Command::new("sh")
    //     .arg("-c")
    //     .arg(cmd)
    //     .stdout(Stdio::piped())
    //     // .stderr(Stdio::piped())
    //     .spawn()?;
    // Create a BufReader for the child's stdout
    // let reader = BufReader::new(child.stdout.take().unwrap());
    // Print each line as it's received
    // for line in reader.lines() {
    //     println!("{}", line?);
    // }
    // Wait for the child process to finish
    // let _ = child.wait()?;
    // Ok(())
    // let stdout = child.stdout.take().unwrap();
    // let stderr = child.stderr.take().unwrap();
    // // Read stdout and stderr in separate threads
    // let stdout_handle = std::thread::spawn(move || read_and_print(stdout));
    // let stderr_handle = std::thread::spawn(move || read_and_print(stderr));
    // stdout_handle.join().unwrap()?;
    // stderr_handle.join().unwrap()?;
    // // Wait for the child process to finish
    // let _ = child.wait()?;
    // Ok(())
}

// fn read_and_print<R: Read>(mut reader: R) -> io::Result<()> {
//     let mut buffer = Vec::new();
//     reader.read_to_end(&mut buffer)?;
//     let output = String::from_utf8_lossy(&buffer).to_string();

//     // Construct the path to the file on the Desktop
//     let mut path = home_dir().unwrap_or(PathBuf::from("."));
//     path.push("Desktop");
//     path.push("output.txt");

//     // Write the output to the file
//     write(&path, &output)?;

//     Command::new("sh").arg("-c").arg("tput reset").status()?;
//     println!("{}", output);
//     Ok(())
// }
