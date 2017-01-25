#[macro_use]
extern crate clap;
extern crate num;

use clap::App;
use clap::SubCommand;

use std::io::stdout;
use std::io::stdin;
use std::io::Read;
use std::io::Write;

mod mimir;
mod token;
mod object;
mod libstd;

use mimir::Mimir;
use token::Token;

fn main() {
    let matches = App::new("mimir")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Mimir language virtual machine")
        .subcommand(SubCommand::with_name("live")
                    .about("Mimir language live interpreter"))
        .subcommand(SubCommand::with_name("build")
                    .about("Load Mimir from a file"))
        .get_matches();

    let mut mimir = Mimir::new();

    if let Some(matches) = matches.subcommand_matches("live") {
        let mut stdout = stdout();
        let mut stdin = stdin();
        loop {
            stdout.write(b">> ");
            stdout.flush();

            let tokens = {
                let mut buffer = String::new();
                stdin.read_line(&mut buffer).expect("io err");
                Token::tokenize(buffer.trim())
            };
            mimir.parse(&tokens);
        }
    } 
    
    if let Some(matches) = matches.subcommand_matches("build") {

    }
}
