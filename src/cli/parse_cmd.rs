use std::io;
use std::io::Write;

use anyhow::Result;
use clap::Parser;
use inquire::{Confirm, Password, PasswordDisplayMode, Text};
use sled::Mode;

use super::*;

pub fn process_cmd() -> Result<()> {
    print_banner();

    // get the dir where password stored
    let mut source_path = dirs::home_dir().expect("home_dir wrong!");
    source_path.push(".rupass");

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
            let url = Text::new("🐳 please input the website/app keyword:")
                .with_help_message("e.g. http://docs.rs   qq")
                .prompt()?;
            let user = Text::new("🥷🏻 please input your username:")
                .with_help_message("e.g. admin")
                .prompt()?;
            let password = Password::new("🌴 please input your password")
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_display_toggle_enabled()
                .with_help_message("you can change the password display by press Ctrl+R.")
                .prompt()?;
            let notes = Text::new("🦀 please input some website notes/hints:")
                .with_help_message("this is a private website")
                .prompt()?;
            let ans = Confirm::new("Are you sure to store this password?")
                .with_default(true)
                .with_help_message("press Enter/y/yes to store, press n/no to cancel.")
                .prompt()?;
            match ans {
                true => {
                    add_pwd(&db, url, user, password, notes)?;
                    eprintln!("save done!")
                }
                false => {
                    eprintln!("cancel!")
                }
            }
        }
        Some(Commands::Search) => {
            let search_str = Text::new("🦀 please input search keyword:")
                .with_help_message("select from the account list.")
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
                    writeln!(print_handle, " website/app: {}\n user: {}\n password: {}\n notes: {}",
                             key.url, key.user, value.password, value.notes)?;
                } else {
                    continue;
                }
            }

            writeln!(print_handle, "===================total: {} records=====================", index)?;
            print_handle.flush()?;
        }
        Some(Commands::Delete) => {
            let delete_keyword = Text::new("🦀 please input/select the entire account info:")
                .with_help_message("suggesting you select from the account list.")
                .with_suggester(&account_suggester)
                .prompt()?;

            // open database
            let db = sled::open(source_path.as_path())?;
            // get all website keywords
            let pwd_map = get_all_to_map(&db)?;

            if let Some(pwd_key) = get_entire_account(&delete_keyword, pwd_map) {
                eprintln!("Delete account info: {}<{}>", pwd_key.url, pwd_key.user);
                let ans = Confirm::new("Are you sure to delete?")
                    .with_default(true)
                    .with_help_message("press Enter/y/yes delete, press n/no to cancel.")
                    .prompt()?;
                match ans {
                    true => {
                        delete_account(&db, &pwd_key.url, &pwd_key.user)?;
                        eprintln!("delete success!")
                    }
                    false => {
                        eprintln!("delete failed!")
                    }
                }
            } else {
                eprintln!("Bad input! You need to input/select the entire account info!");
            }
        }
        Some(Commands::Update) => {
            println!("sorry! this ia a todo()! part.")
        }
        None => {}
    }

    Ok(())
}


