use std::{iter::FilterMap, path::Path};

use walkdir::{DirEntry, FilterEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
	entry
		.file_name()
		.to_str()
		.map(|s| s.contains(".git"))
		.unwrap_or(false)
}
