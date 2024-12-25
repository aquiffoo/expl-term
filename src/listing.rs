use std::fs;
use std::path::PathBuf;

pub fn list_files(
	curr_dir: &PathBuf,
) -> Result<Vec<String>,
std::io::Error> {
	let mut entries = Vec::new();
	if let Ok(dir) = fs::read_dir(curr_dir) {
		for e in dir {
			if let Ok(e) = e {
				let metadata = e.metadata()?;
				let name = e.file_name().to_string_lossy().to_string();
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
