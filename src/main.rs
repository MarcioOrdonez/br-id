use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::domain::cpf::Cpf;

pub mod domain;

#[derive(Parser, Debug)]
enum CliArgs {
    Cpf(CpfOptions),
}

#[derive(Parser, Debug)]
struct CpfOptions {
    clip_board: Option<bool>,
}

fn main() {
    let args = CliArgs::parse();

    match args {
        CliArgs::Cpf(options) => {
            let cpf: String = Cpf::generate();
            match options.clip_board {
                Some(true) => {
                    let mut clipboard =
                        ClipboardContext::new().expect("Error creating clipboard context");
                    clipboard
                        .set_contents(cpf.to_owned())
                        .expect("Error setting clipboard value");
                    cli_clipboard::set_contents(cpf.to_owned())
                        .expect("Error setting clipboard value");
                }
                _ => (),
            }
            println!("{:?}", cpf);
        }
    }
}
