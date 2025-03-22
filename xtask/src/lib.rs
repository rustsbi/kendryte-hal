//! Kendryte K230 development utilities.
//!
//! This crate provides tools and utilities for working with the Kendryte K230 platform,
//! including firmware generation, encryption, and image formatting.

use clap::{Parser, Subcommand};
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
    /// Generate firmware for Kendryte K230.
    GenFirmware {
        /// Input file path.
        #[arg(long = "ifile", short = 'i')]
        input: String,
        /// Output file path (optional).
        #[arg(long = "ofile", short = 'o')]
        output: Option<String>,
        /// Encryption type (optional).
        #[arg(long, short = 'e')]
        encryption: Option<String>,
    },
}
