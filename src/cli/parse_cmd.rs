use std::io;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use clap::Parser;
use dashmap::DashMap;
use inquire::Text;
use sled::{Db, Mode};

use crate::pb::*;

use super::*;

pub fn process_cmd() -> Result<()> {

    // get the dir where password stored
    let mut source_path = dirs::home_dir().expect("home_dir wrong!");
    source_path.push(".rupass");
    // entry password
    let mut entry_password = String::new();

    // print msg BufWriter缓冲
    let stdout = io::stdout();
    let mut print_handle = io::BufWriter::new(stdout);

    // parse command
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => {
            let primary_password = Text::new("🔐 please input your entry password which you must remember:")
                .with_help_message("e.g. 1234#@com456")
                .prompt()?;

            // create and open sled Db
            let db = sled::Config::default()
                .mode(Mode::LowSpace)
                .path(source_path.as_path())
                .open()?;


            add_pwd(&db, "primary_key".to_string(), "root".to_string(), primary_password.clone(), "secret".to_string())?;
            eprintln!("congratulations! you have init your own password repository!\n\
            store dir: {}/, entry password: {}", source_path.to_str().unwrap(), &primary_password);
        }
        Some(Commands::Add) => {
            let db = sled::open(source_path.as_path())?;
            let url = Text::new("🐳 please input the website address:")
                .with_help_message("http://docs.rs")
                .prompt()?;
            let user = Text::new("🥷🏻 please input your username:")
                .with_help_message("admin")
                .prompt()?;
            let password = Text::new("🧌 please input your password:")
                .with_help_message("123Dd@126.com")
                .prompt()?;
            let notes = Text::new("🦀 please input some website notes/hints:")
                .with_help_message("this is a private website")
                .prompt()?;

            add_pwd(&db, url, user, password, notes)?;
            eprintln!("save done!")
        }
        Some(Commands::Search) => {
            let search_str = Text::new("🦀 please input search keyword:")
                .with_help_message("qq")
                .with_suggester(&key_suggester)
                .prompt()?;

            // open database
            let db = sled::open(source_path.as_path())?;
            // get all website keywords
            let pwd_map = get_all_to_map(&db)?;

            writeln!(print_handle, "search:{}, please waiting...", search_str)?;
            let mut index = 0;
            for (key, value) in pwd_map {
                if key.url.contains(search_str.as_str()) {
                    index += 1;
                    writeln!(print_handle, "===================Result NO.{}=====================", index)?;
                    writeln!(print_handle, "website: {}\n user:{}\n password:{}\n notes:{}",
                             key.url, key.user, value.password, value.notes)?;
                } else {
                    continue;
                }
            }

            writeln!(print_handle, "===================total: {} records=====================", index)?;
            print_handle.flush()?;
        }
        Some(Commands::Delete) => {}
        Some(Commands::Update) => {}
        None => {}
    }

    Ok(())
}


