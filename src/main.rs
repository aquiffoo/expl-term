mod listing;
mod ui;

use crossterm::{
	self,
	terminal
};
use listing::list_files;
use ui::run_ui;
use std::{env, result::Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let curr_dir = env::current_dir()?;
	let files = list_files(&curr_dir)?;

	terminal::enable_raw_mode()?;
	run_ui(files)?;
	terminal::disable_raw_mode()?;

	Ok(())
}
