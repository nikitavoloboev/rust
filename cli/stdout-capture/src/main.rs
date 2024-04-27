#![allow(unused_imports)]
#![allow(dead_code)]

use dirs_next::home_dir;
use std::env;
use std::fs::write;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// TODO: run the command passed as is but capture stdout and stderr
// TODO: put it to clipboard automatically
fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // Ensure the command is provided
    if args.len() < 2 {
        println!("Provide a command to run");
        return Ok(());
    }

    let cmd = if let Some(index) = args.iter().position(|arg| arg == "-c") {
        let (first, second) = args.split_at(index);
        format!("{} {}", first.join(" "), second.join(" "))
    } else {
        args[1..].join(" ")
    };

    // The command to run
    // let cmd = &args[1..].join(" ");

    println!("cmd: {:?}", cmd);

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

    Ok(())

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

fn read_and_print<R: Read>(mut reader: R) -> io::Result<()> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let output = String::from_utf8_lossy(&buffer).to_string();

    // Construct the path to the file on the Desktop
    let mut path = home_dir().unwrap_or(PathBuf::from("."));
    path.push("Desktop");
    path.push("output.txt");

    // Write the output to the file
    write(&path, &output)?;

    Command::new("sh").arg("-c").arg("tput reset").status()?;
    println!("{}", output);
    Ok(())
}
