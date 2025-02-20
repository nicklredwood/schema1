use clap::{Parser, Subcommand};
use std::path::PathBuf;
//use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    // /// Set the Schema store to operate on; defaults to the current directory
    // #[arg(short, long)]
    // store: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    
    #[command(subcommand)]
    commands: Option<Commands>,
}


//
// add, edit
//

#[derive(Subcommand, Debug)]
enum Commands {
    
    /// Add files to the Schema store
    Add {

        /// The files to add to the Schema store
        files: Vec<PathBuf>,
    }
}

