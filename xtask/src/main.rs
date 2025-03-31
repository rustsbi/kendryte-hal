use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use xtask::generate::firmware::{gen_firmware, EncryptionType};
use xtask::generate::image::gen_image;
use xtask::{Cli, Command};

/// Main function for the xtask utility.
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::GenFirmware {
            input,
            output,
            encryption,
        } => {
            // defaulting to `None` if not specified
            let encryption = encryption.unwrap_or_default();
            // Read input file
            let input_file = Path::new(&input);

            // Check if input file exists
            if !input_file.is_file() {
                println!("Input file does not exist");
                return;
            };

            // Get input file path
            let path_buf = input_file.to_path_buf();
            let path = match path_buf.to_str() {
                Some(p) => p,
                None => {
                    println!("Failed to get file path");
                    return;
                }
            };

            let (name, _) = split_file_name(path);

            // Read input file contents
            let mut input_file = match File::open(input_file) {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to open input file: {}", e);
                    return;
                }
            };
            let mut input_data = vec![];
            match input_file.read_to_end(&mut input_data) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to read input file: {}", e);
                    return;
                }
            }

            // Generate firmware
            let firmware = match gen_firmware(&input_data, encryption) {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to generate firmware: {}", e);
                    return;
                }
            };

            // Generate firmware image
            let image = match gen_image(&firmware) {
                Ok(i) => i,
                Err(e) => {
                    println!("Failed to generate image: {}", e);
                    return;
                }
            };

            // Close input file
            drop(input_file);

            // Determine output file name
            let output_file = output.unwrap_or_else(|| format!("{}.img", name));

            // Write output file
            let mut output_file = match File::create(output_file) {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to create output file: {}", e);
                    return;
                }
            };
            match output_file.write_all(&image) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to write image: {}", e);
                    return;
                }
            }

            println!("Firmware generation successful!");
        }
    }
}

/// Split a file name into its name and extension parts.
fn split_file_name(file_name: &str) -> (&str, Option<&str>) {
    match file_name.rfind('.') {
        Some(dot_index) => (&file_name[..dot_index], Some(&file_name[dot_index + 1..])),
        None => (file_name, None),
    }
}
