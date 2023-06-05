use std::fs;

use anyhow::{Ok, Result};

pub fn write_desktop_entry() -> Result<()> {
	let desktop_file = directories::BaseDirs::new()
		.expect("Failed to get BaseDirs")
		.data_dir()
		.join("applications")
		.join("wo.desktop");

	let content = "[Desktop Entry]
Encoding=UTF-8
Name=wo
Exec=wo
Icon=/home/edwin/.local/share/JetBrains/Toolbox/apps/Fleet/ch-0/.icon.png
Comment=A tool manager
Version=1.0
Type=Application
Categories=Development
Terminal=false
";

	fs::write(desktop_file, content)?;

	Ok(())
}
