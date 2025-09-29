use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use xtask::convert::elf::{elf_to_bin, elf_to_image};
use xtask::error::XtaskResult;
use xtask::generate::image::gen_image;
use xtask::{Cli, Command};

/// Entry point for the xtask utility.
fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> XtaskResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::GenImage {
            input,
            output,
            encryption,
        } => {
            let output_path = resolve_output_path(&input, output, "img");
            let encryption = encryption.unwrap_or_default();

            let data = fs::read(&input)?;
            let image = gen_image(&data, encryption)?;
            fs::write(&output_path, &image)?;

            println!("Success! Image saved to: {}", output_path.display());
        }
        Command::Elf2Bin { input, output } => {
            let output_path = resolve_output_path(&input, output, "bin");
            elf_to_bin(&input, &output_path)?;

            println!("Success! Binary saved to: {}", output_path.display());
        }
        Command::Elf2Img {
            input,
            output,
            encryption,
        } => {
            let output_path = resolve_output_path(&input, output, "img");
            let encryption = encryption.unwrap_or_default();
            elf_to_image(&input, &output_path, encryption)?;

            println!("Success! Image saved to: {}", output_path.display());
        }
    }

    Ok(())
}

fn resolve_output_path(input: &Path, output: Option<PathBuf>, default_extension: &str) -> PathBuf {
    output.unwrap_or_else(|| input.with_extension(default_extension))
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command as AssertCommand;
    use object::write::{Object as WriteObject, StandardSegment};
    use object::{Architecture, BinaryFormat, Endianness, SectionKind};
    use predicates::prelude::*;
    use std::path::Path;
    use tempfile::{NamedTempFile, TempDir};

    fn build_test_elf() -> Vec<u8> {
        let mut obj =
            WriteObject::new(BinaryFormat::Elf, Architecture::Riscv64, Endianness::Little);
        let segment = obj.segment_name(StandardSegment::Text).to_vec();

        let text_section = obj.add_section(segment.clone(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text_section, b"\x13\x05\x00\x00", 4);

        let data_section = obj.add_section(segment.clone(), b".data".to_vec(), SectionKind::Data);
        obj.append_section_data(data_section, b"\x12\x34\x56\x78", 1);

        let bss_section =
            obj.add_section(segment, b".bss".to_vec(), SectionKind::UninitializedData);
        obj.append_section_bss(bss_section, 8, 1);

        obj.add_file_symbol(b"test".into());

        obj.write().expect("failed to build test ELF")
    }

    fn write_temp_elf(dir: &TempDir, file_name: &str) -> std::path::PathBuf {
        let path = dir.path().join(file_name);
        std::fs::write(&path, build_test_elf()).expect("write elf to disk");
        path
    }

    #[test]
    fn test_gen_image_default_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path();
        std::fs::write(input_path, b"test data")?;

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(input_path);

        let expected_output = input_path.with_extension("img");
        let expected_display = expected_output.display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(expected_output.exists());

        Ok(())
    }

    #[test]
    fn test_gen_image_custom_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let output_file = NamedTempFile::new()?;
        std::fs::write(input_file.path(), b"test data")?;

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("gen-image")
            .arg("--input")
            .arg(input_file.path())
            .arg("--output")
            .arg(output_file.path());

        let expected_display = output_file.path().display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(output_file.path().exists());

        Ok(())
    }

    #[test]
    fn test_gen_image_input_without_extension() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?.into_temp_path();
        let input_path = Path::new(input_file.to_str().unwrap()).with_extension("");
        std::fs::rename(input_file, &input_path)?;
        std::fs::write(&input_path, b"test data")?;

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(&input_path);

        cmd.assert().success();

        let expected_output = input_path.with_extension("img");
        assert!(expected_output.exists());

        Ok(())
    }

    #[test]
    fn test_gen_image_input_with_multiple_extensions() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path().with_extension("tar.gz");
        std::fs::rename(input_file, &input_path)?;
        std::fs::write(&input_path, b"test data")?;

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("gen-image").arg("--input").arg(&input_path);

        cmd.assert().success();

        let expected_output = input_path.with_extension("img");
        assert!(expected_output.exists());

        Ok(())
    }

    #[test]
    fn test_elf2bin_default_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let input_path = write_temp_elf(&dir, "firmware.elf");

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("elf2bin").arg("--input").arg(&input_path);

        let expected_output = input_path.with_extension("bin");
        let expected_display = expected_output.display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(expected_output.exists());
        let contents = std::fs::read(&expected_output)?;
        assert!(!contents.is_empty());

        Ok(())
    }

    #[test]
    fn test_elf2bin_custom_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let input_path = write_temp_elf(&dir, "firmware.elf");
        let output_path = dir.path().join("custom.bin");

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("elf2bin")
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path);

        let expected_display = output_path.display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(output_path.exists());
        let contents = std::fs::read(&output_path)?;
        assert!(!contents.is_empty());

        Ok(())
    }

    #[test]
    fn test_elf2img_default_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let input_path = write_temp_elf(&dir, "firmware.elf");

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("elf2img").arg("--input").arg(&input_path);

        let expected_output = input_path.with_extension("img");
        let expected_display = expected_output.display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(expected_output.exists());
        let contents = std::fs::read(&expected_output)?;
        assert!(!contents.is_empty());

        Ok(())
    }

    #[test]
    fn test_elf2img_custom_output_path() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let input_path = write_temp_elf(&dir, "firmware.elf");
        let output_path = dir.path().join("custom.img");

        let mut cmd = AssertCommand::cargo_bin("xtask")?;
        cmd.arg("elf2img")
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--encryption")
            .arg("aes");

        let expected_display = output_path.display().to_string();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_display));

        assert!(output_path.exists());
        let contents = std::fs::read(&output_path)?;
        assert!(!contents.is_empty());

        Ok(())
    }
}
