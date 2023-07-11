use std::{fs, process::Command};

use anyhow::{Ok, Result};

pub fn write_desktop_entry() -> Result<()> {
	Command::new("desktop-file-install")
		.arg("--dir")
		.arg(directories::BaseDirs::new().expect("Failed to get BaseDirs").data_dir().join("applications"))
		.arg("--rebuild-mime-info-cache")
		.arg("./share/wo.desktop")
		.spawn()
		.unwrap()
		.wait_with_output()
		.unwrap();

	Ok(())
}
