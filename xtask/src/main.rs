use clap::Parser;
use std::fs;
use xtask::generate::image::gen_image;
use xtask::{Cli, Command};

/// Main function for the xtask utility.
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::GenImage {
            input,
            output,
            encryption,
        } => {
            let encryption = encryption.unwrap_or_default();
            let output = output.unwrap_or(input.with_extension("img"));

            let data = match fs::read(input) {
                Ok(data) => data,
                Err(e) => {
                    println!("Failed to read input file: {}", e);
                    return;
                }
            };

            // Generate firmware image
            let image = match gen_image(&data, encryption) {
                Ok(i) => i,
                Err(e) => {
                    println!("Failed to generate image: {}", e);
                    return;
                }
            };

            match fs::write(&output, &image) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to write image: {}", e);
                    return;
                }
            }

            println!("Success! Image saved to: {}", output.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path();
        std::fs::write(input_path, b"test data")?;

        let mut cmd = Command::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(input_path);

        cmd.assert().success().stdout(predicate::str::contains(
            input_path.with_extension("img").display().to_string(),
        ));

        let expected_output = input_path.with_extension("img");
        assert!(expected_output.exists());

        Ok(())
    }

    #[test]
    fn test_custom_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let output_file = NamedTempFile::new()?;
        std::fs::write(input_file.path(), b"test data")?;

        let mut cmd = Command::cargo_bin("xtask")?;
        cmd.arg("gen-image")
            .arg("--input")
            .arg(input_file.path())
            .arg("--output")
            .arg(output_file.path());

        cmd.assert().success().stdout(predicate::str::contains(
            output_file.path().display().to_string(),
        ));

        assert!(output_file.path().exists());

        Ok(())
    }

    #[test]
    fn test_input_without_extension() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?.into_temp_path();
        let input_path = Path::new(input_file.to_str().unwrap()).with_extension("");
        std::fs::rename(input_file, &input_path)?;
        std::fs::write(&input_path, b"test data")?;

        let mut cmd = Command::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(&input_path);

        cmd.assert().success();

        let expected_output = input_path.with_extension("img");
        assert!(expected_output.exists());

        Ok(())
    }

    #[test]
    fn test_input_with_multiple_extensions() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path().with_extension("tar.gz");
        std::fs::rename(input_file, &input_path)?;
        std::fs::write(&input_path, b"test data")?;

        let mut cmd = Command::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(&input_path);

        cmd.assert().success();

        let expected_output = input_path.with_extension("img");
        assert!(expected_output.exists());

        Ok(())
    }
}
