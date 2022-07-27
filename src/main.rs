use anyhow::Result;

use cli::process_cmd;

mod pb;
mod cli;

fn main() -> Result<()> {
    process_cmd()
}
