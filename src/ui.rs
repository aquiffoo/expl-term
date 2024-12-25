use crossterm::{
	cursor,
	execute,
	event::{
		self,
		Event,
		KeyCode,
		KeyEventKind,
	},
	terminal::{
		self,
		ClearType,
	},
	style::{
		PrintStyledContent,
		Color,
		Stylize,
	},
};
use std::{
	io::stdout,
	fs,
	path::PathBuf,
	env,
};

pub fn run_ui(mut files: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = stdout();
	let mut selected = 0;
	let mut history = Vec::new();
	let mut redo_stack: Vec<PathBuf> = Vec::new();
	let mut current_dir = env::current_dir()?;

	execute!(stdout, terminal::EnterAlternateScreen)?;

	loop {
		execute!(
			stdout,
			terminal::Clear(ClearType::All),
			cursor::MoveTo(0, 0)
		)?;

		for (i, f) in files.iter().enumerate() {
			if i == selected {
				execute!(stdout, PrintStyledContent(format!("> {}\n", f).with(Color::Green)))?;
			} else if f.ends_with('/') {
				execute!(stdout, PrintStyledContent(format!("  {}\n", f).with(Color::Blue)))?;
			} else {
				execute!(stdout, PrintStyledContent(format!("  {}\n", f).with(Color::White)))?;
			}
		}

		draw_bottom_bar(&mut stdout, &current_dir)?;

		if let Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Up => {
						if selected > 0 {
							selected -= 1;
						}
					}
					KeyCode::Down => {
						if selected < files.len() - 1 {
							selected += 1;
						}
					}
					KeyCode::Enter => {
						let sel = &files[selected];
						if sel.ends_with('/') {
							current_dir.push(sel.trim_end_matches('/'));
							history.push(current_dir.clone());
							redo_stack.clear();
							files = list_dir(&current_dir)?;
							selected = 0;
						} else {
							let status = std::process::Command::new("vim")
								.arg(current_dir.join(sel))
								.status();

							if let Err(err) = status {
								eprintln!("Error: {}", err);
							}
						}
					}
					KeyCode::Char('q') => {
						break;
					}
					KeyCode::Char('s') => {
						// search
						let mut search_query = String::new();
						execute!(
							stdout,
							cursor::MoveTo(0, terminal::size()?.1 - 2),
							terminal::Clear(ClearType::CurrentLine),
							PrintStyledContent("Search: ".with(Color::Yellow))
						)?;

						loop {
							if let Event::Key(key) = event::read()? {
								if key.kind == KeyEventKind::Press {
									match key.code {
										KeyCode::Char(c) => {
											search_query.push(c);
											execute!(
												stdout,
												cursor::MoveTo(8, terminal::size()?.1 - 2),
												PrintStyledContent(search_query.clone().with(Color::Yellow))
											)?;
										}
										KeyCode::Backspace => {
											search_query.pop();
											execute!(
												stdout,
												cursor::MoveTo(8, terminal::size()?.1 - 2),
												terminal::Clear(ClearType::UntilNewLine),
												PrintStyledContent(search_query.clone().with(Color::Yellow))
											)?;
										}
										KeyCode::Enter => {
											if !search_query.is_empty() {
												selected = files.iter()
													.position(|f| f.to_lowercase().contains(&search_query.to_lowercase()))
													.unwrap_or(selected);
											}
											break;
										}
										KeyCode::Esc => break,
										_ => {}
									}
								}
							}
						}
					}
					KeyCode::Left => {
						if let Some(parent) = current_dir.parent() {
							current_dir = parent.to_path_buf();
							history.push(current_dir.clone());
							redo_stack.clear();
							files = list_dir(&current_dir)?;
							selected = 0;
						}

					}
					KeyCode::Right => {
						if let Some(next) = redo_stack.pop() {
							history.push(current_dir.clone());
							current_dir = next;
							files = list_dir(&current_dir)?;
							selected = 0;
						}
					}   
					_ => {}
				}
			}
		}
	}

	execute!(stdout, terminal::LeaveAlternateScreen)?;

	Ok(())
}

fn list_dir(dir: &PathBuf) -> Result<Vec<String>, std::io::Error> {
	let mut entries = Vec::new();
	if let Ok(read_dir) = fs::read_dir(dir) {
		for entry in read_dir {
			if let Ok(entry) = entry {
				let metadata = entry.metadata()?;
				let name = entry.file_name().to_string_lossy().to_string();
				if metadata.is_dir() {
					entries.push(format!("{}/", name));
				} else {
					entries.push(name);
				}
			}
		}
	}
	Ok(entries)
}

fn draw_bottom_bar(stdout: &mut std::io::Stdout, current_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
	let width = terminal::size()?.0 as usize;
	
	let dir_str = current_dir.to_str().unwrap_or("");

	let bar_content = format!(
		"Current dir: {}",
		dir_str
	);
	
	execute!(stdout, cursor::MoveTo(0, terminal::size()?.1 as u16 - 1))?;
	
	if bar_content.len() > width {
		let truncated_content = &bar_content[..width];
		execute!(stdout, PrintStyledContent(truncated_content.with(Color::Cyan)))?;
	} else {
		execute!(stdout, PrintStyledContent(bar_content.clone().with(Color::Cyan)))?;
	}

	let empty_space = " ".repeat(width - bar_content.len());
	execute!(stdout, PrintStyledContent(empty_space.with(Color::Black)))?;

	Ok(())
}
