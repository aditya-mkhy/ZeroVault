mod crypto;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use zeroize::Zeroize;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "zerovault")]
#[command(about = "ZeroVault - zero-knowledge encrypted cloud storage", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Decrypta file
    Decrypt {
        /// Input file path 
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path 
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output } => {
            // Ask for password twice
            let mut password = rpassword::prompt_password("Enter password  : ")?;
            let mut confirm = rpassword::prompt_password("Confirm password: ")?;

            if password != confirm {
                eprintln!("Error: passwords do not match.");
                password.zeroize();
                confirm.zeroize();
                return Ok(());
            }

            confirm.zeroize();
            println!("Encrypted '{}' → '{}' with pass -> '{}'", input.display(), output.display(), password);

            // encrypt file
            crypto::encrypt_file_with_password(&input, &output, &password)?;
            
            // Wipe password
            password.zeroize();

        }

        Commands::Decrypt { input, output } => {
            let mut password = rpassword::prompt_password("Password: ")?;
            println!("Decrypted '{}' → '{}' with pass -> '{}'", input.display(), output.display(), password);
            password.zeroize();
        }
    }

    Ok(())

}
