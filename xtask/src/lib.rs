//! Kendryte K230 development utilities.
//!
//! This crate provides tools and utilities for working with the Kendryte K230 platform,
//! including firmware generation, encryption, and image formatting.

extern crate core;

use crate::generate::image::EncryptionType;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod error;
pub mod generate;

/// CLI structure for the xtask utility.
#[derive(Parser, Debug)]
#[clap(name = "xtask", about = "A utility for Kendryte K230 development")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Subcommands for the xtask utility.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate image for Kendryte K230.
    ///
    /// Ref: https://github.com/kendryte/canmv_k230/blob/main/tools/firmware_gen.py
    GenImage {
        /// Input file path.
        #[arg(long = "input", short = 'i')]
        input: PathBuf,
        /// Output file path (optional).
        ///
        /// Using default output path
        ///
        ///     cargo xtask  gen-image -i target/riscv64gc-unknown-none-elf/release/uart-demo.bin
        ///
        ///     Output: target/riscv64gc-unknown-none-elf/release/uart-demo.img.
        ///
        /// Specifying custom output path
        ///
        ///     cargo xtask  gen-image -i target/riscv64gc-unknown-none-elf/release/uart-demo.bin -o ./uart-demo.x
        ///
        ///     Output: ./uart-demo.x
        #[arg(long = "output", short = 'o')]
        output: Option<PathBuf>,
        /// Encryption type (optional).
        ///
        /// Parameter:
        ///
        ///     none: NO ENCRYPTION + HASH-256 (default)
        ///
        ///     sm4: SM4-CBC + SM2
        ///
        ///     aes: AES-GCM + RSA-2048
        #[arg(long, short = 'e')]
        encryption: Option<EncryptionType>,
    },
}
