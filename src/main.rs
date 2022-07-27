extern crate core;

use std::borrow::Borrow;
use std::io::{BufWriter, stdin, stdout, Write};

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use clap::Parser;
use inquire::Text;
use sled::Mode;

use cli::*;
use pb::*;

mod pb;

mod cli;

fn main() -> Result<()> {
    // get the dir where password stored
    let mut source_path = dirs::home_dir().expect("home_dir wrong!");
    source_path.push(".rupass");
    // entry password
    let mut entry_password = String::new();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            let entry_password = Text::new("🔐 please input your entry password which you must remember:")
                .with_help_message("e.g. 1234#@com456")
                .prompt()?;

            // create and open sled Db
            let db = sled::Config::default()
                .mode(Mode::LowSpace)
                .path(source_path.as_path())
                .open()?;

            db.insert("entry", entry_password.as_str())?;
            eprintln!("congratulations! you have init your own password repository!\n\
            store dir: {}/, entry password: {}", source_path.to_str().unwrap(), &entry_password);
        }
        Some(Commands::Add) => {
            let db = sled::open(source_path.as_path())?;
            let url = Text::new("🐳 please input the website address:")
                .with_help_message("http://docs.rs")
                .prompt()?;
            let username = Text::new("🥷🏻 please input your username:")
                .with_help_message("admin")
                .prompt()?;
            let password = Text::new("🧌 please input your password:")
                .with_help_message("123Dd@126.com")
                .prompt()?;
            let notes = Text::new("🦀 please input some website notes/hints:")
                .with_help_message("this is a private website")
                .prompt()?;

            let key = PwdKey::new(url, username);
            let key_u8: Bytes = key.into();
            let info = PwdInfo::new(password, notes);
            let info_u8: Bytes = info.into();
            let info_u8 = info_u8.to_vec();
            db.insert(key_u8, info_u8)?;
            eprintln!("save done!")
        }
        Some(Commands::Search) => {
            let search_str = Text::new("🦀 please input search keyword:")
                .with_help_message("qq")
                .prompt()?;
            println!("search:{}, please waiting...", search_str);
            println!("===================Result=====================");
            let db = sled::open(source_path.as_path())?;
            for item in db.iter() {
                match item {
                    Ok((key, value)) => {
                        let key_b = BytesMut::from(key.to_vec().as_slice());
                        let key = PwdKey::try_from(key_b)?;
                        let value_b = BytesMut::from(value.to_vec().as_slice());
                        let value = PwdInfo::try_from(value_b)?;
                        if key.url.contains(search_str.as_str()) {
                            println!("website: {}\n user:{}\n password:{}\n notes:{}",
                                     key.url, key.user, value.password, value.notes);
                            println!("=================================================");
                        }
                    }

                    Err(e) => {
                        println!("sorry! error: {}", e);
                    }
                }
            }
        }
        Some(Commands::Delete) => {}
        Some(Commands::Update) => {}
        None => {}
    }

    Ok(())
}
// 774999
