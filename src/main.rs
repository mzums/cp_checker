use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let input_dir = "inputs";
    let output_dir = "outputs";
    let solution_file = "solution.cpp";
    let executable_file = "./solution";

    println!("Compiling {}...", solution_file);
    let compile_status = Command::new("g++")
        .args(&["-o", executable_file, solution_file])
        .status()?;
    if !compile_status.success() {
        eprintln!("Compilation failed!");
        return Ok(());
    }
    println!("Compilation successful!");

    let input_files = fs::read_dir(input_dir)?;
    for entry in input_files {
        let entry = entry?;
        let input_path = entry.path();
        let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
        let output_path = Path::new(output_dir).join(format!("{}.out", file_stem));

        println!("Running test case: {}", input_path.display());

        let output = Command::new(executable_file)
            .stdin(File::open(&input_path)?)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            eprintln!("Test case {} failed to execute!", file_stem);
            continue;
        }

        let program_output = String::from_utf8_lossy(&output.stdout);
        let expected_output = fs::read_to_string(&output_path)?;

        if program_output.trim() == expected_output.trim() {
            println!("Test case {} passed!", file_stem);
        } else {
            println!("Test case {} failed!", file_stem);
            println!("Expected:\n{}", expected_output);
            println!("Got:\n{}", program_output);
        }
    }

    fs::remove_file(executable_file)?;
    println!("All test cases completed.");

    Ok(())
}
